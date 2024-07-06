use tokio::sync::mpsc::UnboundedReceiver;
use tracing::info;

use super::models::orderbook::LiveOrderBookMessage;

#[derive(Debug)]
pub struct OrderBookManager {}

impl OrderBookManager {
    pub async fn listen_order_messages(&self, mut receiver: UnboundedReceiver<LiveOrderBookMessage>) {
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                info!("order message = {:?}", message);
            }
        });
    }
}
