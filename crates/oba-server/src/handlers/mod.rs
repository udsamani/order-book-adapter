use std::collections::BTreeMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod instruments;

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

#[derive(Debug, Serialize)]
pub struct BestBidResponse {
    pub best_bid: u64,
}

#[derive(Debug, Serialize)]
pub struct BestAskResponse {
    best_ask: u64,
}

#[derive(Debug)]
pub enum OrderBookAdapterError {
    InternalServerError,
    NotFound(String),
}

impl IntoResponse for OrderBookAdapterError {
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
