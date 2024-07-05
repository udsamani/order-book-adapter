
use axum::extract::Path;
use tracing::info;

pub async fn subscribe_instrument(Path(name): Path<String>) {
    info!("Subscribe to {}", name)
}
