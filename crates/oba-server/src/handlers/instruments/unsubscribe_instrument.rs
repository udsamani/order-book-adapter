use axum::extract::Json;
use tracing::info;

use super::UnsubscribeInstrumentRequest;

pub async fn unsubscribe_instrument(Json(request): Json<UnsubscribeInstrumentRequest>) {
    info!("Unsubscribe  {:?}", request);
}
