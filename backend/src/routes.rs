use std;
use std::sync::Arc;
use warp;
use warp::Filter;
use mongodb::Client;

use super::Result;
use crate::reject::handle_rejection;
use crate::handler::*;

macro_rules! reply_future {
    ($function:ident) => {{
        | db: Arc<Client> | async move  {
            match $function(db).await {
                Ok(output) => Ok(warp::reply::json(&output)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        }}
    };
}

macro_rules! reply_future_with_query {
    ($function:ident) => {{
        | query, db: Arc<Client> | async move  {
            match $function(query, db).await {
                Ok(output) => Ok(warp::reply::json(&output)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        }}
    };
}


pub async fn create_routes(db: Arc<Client>) -> Result<impl warp::Filter<Extract = impl warp::Reply> + Clone> {

    let with_db = warp::any().map(move || db.clone());

    let log_filter = warp::log("api");

    let route_get_last_wishlist = warp::get()
        .and(warp::path("api"))
        .and(warp::path("wishlist"))
        .and(warp::path("last"))
        .and(warp::path::end())
        .and(with_db.clone())
        .and_then(reply_future!(handle_get_last_wishlist));

    let route_get_newest_products = warp::get()
        .and(warp::path("api"))
        .and(warp::path("product"))
        .and(warp::path("newest"))
        .and(warp::path::end())
        .and(with_db.clone())
        .and_then(reply_future!(handle_get_newest_products));

    let route_get_archived_products = warp::get()
        .and(warp::path("api"))
        .and(warp::path("product"))
        .and(warp::path("archive"))
        .and(warp::path::end())
        .and(warp::query())
        .and(with_db.clone())
        .and_then(reply_future_with_query!(handle_get_archived_products));

    let route_get_products_by_category_name = warp::get()
        .and(warp::path("api"))
        .and(warp::path("product"))
        .and(warp::path("category"))
        .and(warp::path::end())
        .and(warp::query())
        .and(with_db.clone())
        .and_then(reply_future_with_query!(handle_get_products_by_category_name));

    let route_get_categories = warp::get()
        .and(warp::path("api"))
        .and(warp::path("category"))
        .and(warp::path("list"))
        .and(warp::path::end())
        .and(with_db.clone())
        .and_then(reply_future!(handle_get_categories));

    let routes = route_get_last_wishlist
        .or(route_get_newest_products)
        .or(route_get_archived_products)
        .or(route_get_products_by_category_name)
        .or(route_get_categories)
        .recover(handle_rejection)
        .with(log_filter);

    Ok(routes)
}
