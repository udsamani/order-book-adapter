use axum::{extract::{Path, State}, Json};
use tracing::info;

use crate::AppState;

use super::{BestBidResponse, OrderBookAdapterError, OrderBookResponse, SubscribeInstrumentRequest, UnsubscribeInstrumentRequest};


pub async fn get_order_book(State(state): State<AppState>, Path(instrument_name): Path<String>) -> Result<Json<OrderBookResponse>, OrderBookAdapterError>{
    let order_book_manager = state.order_book_manager.read().await;
    let order_book = order_book_manager.get_order_book(&instrument_name);

    match order_book {
        Some(ob) => {
            let order_book_response = OrderBookResponse{
                symbol: instrument_name.to_string(),
                bids: ob.get_bids().clone(),
                asks: ob.get_asks().clone(),
            };
            Ok(Json(order_book_response))
        },
        None => {
            Err(OrderBookAdapterError::NotFound(instrument_name))
        }
    }
}

pub async fn subscribe_instrument(
    State(state): State<AppState>,
    Json(request): Json<SubscribeInstrumentRequest>,
) {
    info!("Subscribe to {:?}", request);
    let mut order_book_manager = state.order_book_manager.write().await;
    order_book_manager.subscribe_instrument(request.name, request.order_book_depth);
}

pub async fn unsubscribe_instrument(
    State(state): State<AppState>,
    Json(request): Json<UnsubscribeInstrumentRequest>) {

    info!("Unsubscribe  {:?}", request);
    let mut order_book_manager = state.order_book_manager.write().await;
    order_book_manager.unsubscribe_instrument(request.name);
}

pub async fn get_best_bid(
    State(state): State<AppState>,
    Path(instrument_name): Path<String>) -> Result<Json<BestBidResponse>, OrderBookAdapterError> {

    info!("fetching best bid for {}", instrument_name);
    let order_book_manager = state.order_book_manager.read().await;
    let best_bid = order_book_manager.get_best_bid(instrument_name);

    let response = BestBidResponse{
        best_bid
    };

    Ok(Json(response))
}
