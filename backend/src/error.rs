use std;
use thiserror::Error;

use bson::document::ValueAccessError;
use bson::oid::Error as BsonError;
use mongodb::error::Error as MongoError;

use crate::model::ErrorMessage;
use crate::reject::get_internal_error_message;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MongoDB: {source}")]
    MongoDB {
        #[from]
        source: MongoError,
    },
    #[error("BSON: {source}")]
    Bson {
        #[from]
        source: BsonError,
    },
    #[error("ValueAccess: {source}")]
    ValueAccess {
        #[from]
        source: ValueAccessError,
    },
    #[error("Received an empty result")]
    EmptyResult,
    #[error("Persistence: Field not loaded: '{0}' is missing '{1}'")]
    FieldNotLoaded(&'static str, &'static str),
}

impl warp::reject::Reject for Error {}

impl Into<ErrorMessage> for &Error {
    fn into(self) -> ErrorMessage {
        match self {
            Error::EmptyResult => ErrorMessage {
                code: 200,
                message: "Empty Result".to_string(),
            },
            _ => get_internal_error_message(),
        }
    }
}
