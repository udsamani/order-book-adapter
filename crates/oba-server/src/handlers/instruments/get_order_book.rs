use axum::{extract::{Path, State}, Json};

use crate::AppState;

use super::{OrderBookError, OrderBookResponse};

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
