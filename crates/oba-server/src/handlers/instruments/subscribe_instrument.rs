
use axum::extract::Json;
use tracing::info;

use super::SubscribeInstrumentRequest;

pub async fn subscribe_instrument(Json(request): Json<SubscribeInstrumentRequest>) {
    info!("Subscribe to {:?}", request)
}
