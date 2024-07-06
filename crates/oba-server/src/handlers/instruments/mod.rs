use std::collections::BTreeMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
pub use get_order_book::get_order_book;
use serde::{Deserialize, Serialize};
use serde_json::json;
pub use subscribe_instrument::subscribe_instrument;
pub use unsubscribe_instrument::unsubscribe_instrument;

mod get_order_book;
mod subscribe_instrument;
mod unsubscribe_instrument;

#[derive(Debug, Deserialize)]
pub struct SubscribeInstrumentRequest {
    name: String,
    order_book_depth: usize,
}

#[derive(Debug, Deserialize)]
pub struct UnsubscribeInstrumentRequest {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct OrderBookResponse {
    symbol: String,
    bids: BTreeMap<u64, f64>,
    asks: BTreeMap<u64, f64>,
}

#[derive(Debug)]
pub enum OrderBookError {
    InternalServerError,
    NotFound(String),
}

impl IntoResponse for OrderBookError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg) = match self {
            Self::NotFound(instrument) => (
                StatusCode::NOT_FOUND,
                format!(
                    "Instrument with name {} not found in order books",
                    instrument
                ),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
        };

        (status, Json(json!({"error_message": error_msg}))).into_response()
    }
}
