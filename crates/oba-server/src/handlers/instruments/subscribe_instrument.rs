use axum::extract::{Json, State};
use tracing::info;

use crate::AppState;

use super::SubscribeInstrumentRequest;

pub async fn subscribe_instrument(
    State(state): State<AppState>,
    Json(request): Json<SubscribeInstrumentRequest>,
) {
    info!("Subscribe to {:?}", request);
    let t = state.bitstamp_instrument_order_provider.as_ref();
    t.subscriber_instrument(request.name)
}
