use axum::extract::{Json, State};
use tracing::info;

use crate::AppState;

use super::UnsubscribeInstrumentRequest;

pub async fn unsubscribe_instrument(
    State(state): State<AppState>,
    Json(request): Json<UnsubscribeInstrumentRequest>) {

    info!("Unsubscribe  {:?}", request);
    let mut order_book_manager = state.order_book_manager.write().await;
    order_book_manager.unsubscribe_instrument(request.name);
}
