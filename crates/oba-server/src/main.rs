use std::{net::SocketAddr, sync::Arc};

use bitstamp::instrument_order_provider::BitstampInstrumentOrderProvider;
use domain::{
    models::LiveOrderBookMessage,
    order_book_manager::{process_order_book_messages, OrderBookManager},
};
use routes::oba_router;
use tokio::sync::{mpsc::unbounded_channel, RwLock};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::config;

mod bitstamp;
mod config;
mod domain;
mod handlers;
mod routes;

#[derive(Clone)]
pub struct AppState {
    order_book_manager: Arc<RwLock<OrderBookManager>>,
}

#[tokio::main]
async fn main() {
    init_tracing();
    let config = config().await;

    let (tx_s, rx_s) = unbounded_channel();

    let bitstamp_instrument_order_provider =
        BitstampInstrumentOrderProvider::new(String::from("wss://ws.bitstamp.net"));

    let (tx, rx) = unbounded_channel::<LiveOrderBookMessage>();
    let order_book_manager = OrderBookManager::new(tx_s);
    let order_book_manager = Arc::new(RwLock::new(order_book_manager));

    let obm_clone = order_book_manager.clone();
    bitstamp_instrument_order_provider.connect(rx_s, tx).await;
    tokio::spawn(async move {
        process_order_book_messages(obm_clone, rx).await;
    });

    let app_state = AppState {
        order_book_manager: order_book_manager.clone(),
    };
    let app = oba_router(app_state);

    let host = config.server_host();
    let port = config.server_port();
    let address: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    info!("Listening on http:{}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
}
