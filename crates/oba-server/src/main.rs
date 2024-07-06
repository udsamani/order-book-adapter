use std::{net::SocketAddr, sync::Arc};

use bitstamp::instrument_order_provider::BitstampInstrumentOrderProvider;
use domain::{models::orderbook::LiveOrderBookMessage, order_book_manager::OrderBookManager};
use routes::oba_router;
use tokio::sync::mpsc::unbounded_channel;
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
    bitstamp_instrument_order_provider: Arc<BitstampInstrumentOrderProvider>,
}

#[tokio::main]
async fn main() {
    init_tracing();
    let config = config().await;

    let (tx_s, rx_s) = unbounded_channel();
    let (tx_su, rx_su) = unbounded_channel();

    let bitstamp_instrument_order_provider =
        BitstampInstrumentOrderProvider::new(String::from("wss://ws.bitstamp.net"), tx_s, tx_su);
    let order_book_manager = OrderBookManager {};
    let (tx, rx) = unbounded_channel::<LiveOrderBookMessage>();
    bitstamp_instrument_order_provider.connect(rx_s, tx).await;
    order_book_manager.listen_order_messages(rx).await;

    let app_state = AppState {
        bitstamp_instrument_order_provider: Arc::new(bitstamp_instrument_order_provider),
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
