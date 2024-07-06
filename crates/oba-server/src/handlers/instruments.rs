use axum::{extract::{Path, State}, Json};
use tracing::info;

use crate::AppState;

use super::{OrderBookError, OrderBookResponse, SubscribeInstrumentRequest, UnsubscribeInstrumentRequest};


pub async fn get_order_book(State(state): State<AppState>, Path(instrument_name): Path<String>) -> Result<Json<OrderBookResponse>, OrderBookError>{
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
            Err(OrderBookError::NotFound(instrument_name))
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
