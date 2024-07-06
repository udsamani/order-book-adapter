use axum::extract::{Path, State};
use tracing::info;

use crate::AppState;

pub async fn get_order_book(State(state): State<AppState>, Path(instrument_name): Path<String>) {
    info!("Get Order Book {}", instrument_name);
    let order_book_manager = state.order_book_manager.read().await;
    order_book_manager.get_order_books();
}
