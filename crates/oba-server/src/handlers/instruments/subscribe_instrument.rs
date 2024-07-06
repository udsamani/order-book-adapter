use axum::extract::{Json, State};
use tracing::info;

use crate::AppState;

use super::SubscribeInstrumentRequest;

pub async fn subscribe_instrument(
    State(state): State<AppState>,
    Json(request): Json<SubscribeInstrumentRequest>,
) {
    info!("Subscribe to {:?}", request);
    let mut order_book_manager = state.order_book_manager.write().await;
    order_book_manager.subscribe_instrument(request.name, request.order_book_depth);
}
