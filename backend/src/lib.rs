extern crate mongodb;
extern crate warp;
#[macro_use]
extern crate log;
extern crate bson;
extern crate thiserror;

mod error;
mod handler;
mod model;
mod query;
mod reject;
mod routes;

pub use self::error::{Error, Result};
pub use self::routes::create_routes;
