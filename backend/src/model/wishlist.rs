use mongodb::bson::{document::Document, oid::ObjectId};
use serde::Serialize;
use std::iter::Iterator;

use super::Product;

#[derive(Serialize, Clone, Debug)]
pub struct Wishlist {
    #[serde(skip)]
    id: Option<ObjectId>,
    timestamp: Option<i32>,
    #[serde(skip)]
    product_ids: Option<Vec<ObjectId>>,
    products: Option<Vec<Product>>,
}

impl Wishlist {
    pub fn get_product_ids(&self) -> Option<&[ObjectId]> {
        self.product_ids.as_ref().map(|e| e.as_slice())
    }
    pub fn get_products(&self) -> Option<&[Product]> {
        self.products.as_ref().map(|e| e.as_slice())
    }
    pub fn set_products(&mut self, products: Vec<Product>) {
        self.products = Some(products);
    }

    pub fn get_timestamp(&self) -> Option<i32> {
        self.timestamp
    }
}

impl From<&Document> for Wishlist {
    fn from(doc: &Document) -> Self {
        Self {
            id: doc.get_object_id("_id").map(|id| id.clone()).ok(),
            timestamp: doc.get_i32("timestamp").ok(),
            product_ids: doc
                .get_array("products")
                .map(|list| {
                    list.into_iter()
                        .filter_map(|e| e.as_object_id().map(|id| id.clone()))
                        .collect()
                })
                .ok(),
            products: None,
        }
    }
}
