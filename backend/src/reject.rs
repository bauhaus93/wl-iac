use std::convert::Infallible;
use warp::http::StatusCode;

use crate::model::ErrorMessage;
use crate::Error;

pub async fn handle_rejection(rej: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let msg;
    if rej.is_not_found() {
        msg = get_not_found_message();
    } else if let Some(err) = rej.find::<Error>() {
        warn!("{}", err);
        msg = err.into();
    } else if let Some(err) = rej.find::<warp::filters::body::BodyDeserializeError>() {
        info!("BodyDeserializeError: {}", err);
        msg = get_bad_request_message();
    } else {
        error!("Unhandeled internal error");
        msg = get_internal_error_message();
    }
    let json = warp::reply::json(&msg);
    Ok(warp::reply::with_status(
        json,
        StatusCode::from_u16(msg.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
    ))
}

fn get_not_found_message() -> ErrorMessage {
    ErrorMessage {
        code: 404,
        message: "Page not found".to_string(),
    }
}

fn get_bad_request_message() -> ErrorMessage {
    ErrorMessage {
        code: 400,
        message: "Bad request".to_string(),
    }
}

pub fn get_internal_error_message() -> ErrorMessage {
    ErrorMessage {
        code: 500,
        message: "Internal server error".to_string(),
    }
}
