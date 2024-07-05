use std::net::SocketAddr;

use routes::oba_router;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::config;


mod config;
mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    init_tracing();
    let config = config().await;
    let app = oba_router();

    let host = config.server_host();
    let port = config.server_port();
    let address: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    info!("Listening on http:{}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
