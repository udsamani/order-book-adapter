use axum::extract::Path;
use tracing::info;



pub async fn unsubscribe_instrument(Path(name): Path<String>) {
    info!("Unsubscribe  {}", name);
}
