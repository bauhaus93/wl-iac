use std;
use std::collections::btree_map::{Entry, BTreeMap};
use mongodb::{bson::{doc, oid::ObjectId, document::Document} , options::{FindOptions, FindOneOptions}, Client, Cursor, Collection};
use std::sync::Arc;
use tokio::stream::StreamExt;

use super::{Result, Error};
use crate::query::{CategoryQuery, ListQuery};
use crate::model::{Category, Source, Wishlist, Product};

pub async fn handle_get_last_wishlist(client: Arc<Client>) -> Result<Wishlist> {
    let mut last_wishlist = get_last_wishlist(&client).await?;
    load_wishlist(&client, &mut last_wishlist).await?;
    Ok(last_wishlist)
}

pub async fn handle_get_newest_products(client: Arc<Client>) -> Result<Vec<Product>> {
    let mut product_list = Vec::new();

    let mut c = 0;
    let mut i = 0;
    while c < 3 && i < 10 {
        let mut last_wishlist = get_nth_wishlist_reverse(&client, i).await?;
        load_wishlist(&client, &mut last_wishlist).await?;

        let wl_timestamp = last_wishlist.get_timestamp();
        let new_products = match last_wishlist.get_products() {
            Some(products) => {
                products.iter().filter(|p| p.get_first_seen() == wl_timestamp)
                .
                fold(Vec::new(), |mut acc, e| {
                    acc.push(e.clone());
                    acc
                })
            },
            None => {
                Vec::new()
            }
        };
        if new_products.len() > 0 {
            c+= 1;
            product_list.extend(new_products);
        }
        i+= 1;
    }
    Ok(product_list)
}

pub async fn handle_get_archived_products(list: ListQuery, client: Arc<Client>) -> Result<Vec<Product>> {
    let last_wishlist = get_last_wishlist(&client).await?;
    let product_ids = last_wishlist
        .get_product_ids()
        .ok_or(Error::FieldNotLoaded("wishlist", "product_ids"))?;
    let filter = doc! {
    "_id": {"$not": {"$in": product_ids} } };

    let options = FindOptions::builder()
        .sort(doc! { "_id": -1})
        .projection(doc! {"_id": false, "item_id": false})
        .skip(list.get_offset() as i64)
        .limit(list.get_size() as i64)
        .build();
    load_products(&client, Some(filter), Some(options)).await
}

pub async fn handle_get_archive_product_count(client: Arc<Client>) -> Result<u64> {
    let last_wishlist = get_last_wishlist(&client).await?;
    let product_ids = last_wishlist
        .get_product_ids()
        .ok_or(Error::FieldNotLoaded("wishlist", "product_ids"))?;
    let filter = doc! {
    "_id": {"$not": {"$in": product_ids} } };

    count_documents(&client.database("wishlist").collection("product"), Some(filter)).await
}

pub async fn handle_get_categories(client: Arc<Client>) -> Result<Vec<Category>> {
    get_categories(&client).await
}

pub async fn handle_get_products_by_category_name(query: CategoryQuery, client: Arc<Client>) -> Result<Vec<Product>> {
    get_products_by_category_name(&client, query.get_category()).await
}

async fn count_documents(collection: &Collection, filter: Option<Document>) -> Result<u64> {
    collection.count_documents(filter, None).await
        .map(|n| n as u64)
        .map_err(Error::from)
}

async fn extract_cursor_results<T: From<Document> + std::fmt::Debug>(mut cursor: Cursor) -> Vec<T> {
    let mut results = Vec::new();
    while let Some(entry) = cursor.next().await {
        if let Ok(result) = entry {
            results.push(T::from(result));
        } else {
            warn!("Couldn't extract bson result as Product");
        }
    }
    results
}


async fn get_categories(client: &Client) -> Result<Vec<Category>> {
    let coll = client.database("wishlist").collection("category");
    let cursor = coll.find(None, None).await?;
    let categories = extract_cursor_results(cursor).await;
    Ok(categories)
}

async fn get_category_by_name(client: &Client, name: &str) -> Result<Category> {
    let coll = client.database("wishlist").collection("category");
    let filter = doc! {
        "name": name
    };
    coll.find_one(Some(filter), None).await
        .map_err(Error::from)
        .and_then(|r| r.ok_or(Error::EmptyResult))
        .map(|r| Category::from(&r))
}

async fn get_products_by_category_name(client: &Client, name: Option<&str>) -> Result<Vec<Product>> {
    let category = match name {
        Some(n) => Some(get_category_by_name(client,n).await?),
        None => None
    };
    get_products_by_category(client, category.as_ref()).await
}

async fn get_products_by_category(client: &Client, category: Option<&Category>) -> Result<Vec<Product>> {
    let filter = match category {
        Some(c) => doc! {
            "category": c.get_id().ok_or(Error::FieldNotLoaded("wishlist", "product_ids"))?,
        },
        None => doc! {
            "category": mongodb::bson::Bson::Null
        }
    };
    load_products(client, Some(filter), None).await
}

async fn get_wishlist(client: &Client, filter: Option<Document>, options: Option<FindOneOptions>) -> Result<Wishlist> {
    let coll = client.database("wishlist").collection("wishlist");
    coll.find_one(filter, options).await
        .map_err(Error::from)
        .and_then(|r| r.ok_or(Error::EmptyResult))
        .map(|r| Wishlist::from(&r))
}

async fn get_nth_wishlist_reverse(client: &Client, skip_count: i64) -> Result<Wishlist> {
    let options = FindOneOptions::builder()
        .sort(doc! {"timestamp": -1})
        .skip(Some(skip_count))
        .projection(doc! {"_id": false})
        .build();
    get_wishlist(client, None, Some(options)).await
}

async fn get_last_wishlist(client: &Client) -> Result<Wishlist> {
    get_nth_wishlist_reverse(client, 0).await
}

async fn load_wishlist(client: &Client, wishlist: &mut Wishlist) -> Result<()> {
    let mut products = match wishlist.get_product_ids() {
        Some(ids) => get_products_by_id(&client, ids).await?,
        None => {
            return Err(Error::FieldNotLoaded("wishlist", "product_ids"));
        }
    };
    load_source_for_products(&client, &mut products).await?;
    wishlist.set_products(products);
    Ok(())
}

async fn load_products(client: &Client, filter: Option<Document>, options: Option<FindOptions>) -> Result<Vec<Product>> {
    let coll = client.database("wishlist").collection("product");
    let cursor = coll.find(filter, options).await?;
    let mut products = extract_cursor_results(cursor).await;
    load_source_for_products(&client, &mut products).await?;
    Ok(products)
}

async fn load_source_for_products(client: &Client, products: &mut [Product]) -> Result<()> {
    let mut sources = BTreeMap::new();
        for product in products.iter_mut() {
            let source_id = product
                .get_source_id()
                .map(|sid| sid.clone())
                .ok_or(Error::FieldNotLoaded("product", "source_id"))?;
            match sources.entry(source_id.clone()) {
                Entry::Vacant(e) => {
                    let source = e.insert(get_source_by_id(client, &source_id).await?);
                    product.set_source(source.clone());
                }
                Entry::Occupied(e) => {
                    let source = e.get().clone();
                    product.set_source(source);
                }
            }
        }

    Ok(())
}

async fn get_source_by_id(client: &Client, id: &ObjectId) -> Result<Source> {
        let coll = client.database("wishlist").collection("source");

        coll.find_one(Some(doc! {"_id": id}), None).await
            .map_err(Error::from)
            .and_then(|r| r.ok_or(Error::EmptyResult).map(|r| Source::from(&r)))
}

async fn get_products_by_id(client: &Client, product_ids: &[ObjectId]) -> Result<Vec<Product>> {
    let coll = client.database("wishlist").collection("product");
        let filter = doc! {
            "_id": { "$in": product_ids}
        };
        let options = FindOptions::builder()
            .sort(doc! {"timestamp": -1})
            .projection(doc! {"_id": false, "item_id": false})
            .build();

    let cursor = coll.find(Some(filter), Some(options)).await?;
    Ok(extract_cursor_results(cursor).await)
}
