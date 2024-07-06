use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
};
use tracing::info;

use crate::bitstamp::models::{BitstampPublicChannel, BitstampRequest, BitstampRequestEvent};

use super::models::orderbook::{LiveOrderBookMessage, OrderBook};

#[derive(Debug)]
pub struct OrderBookManager {
    order_books: HashMap<String, OrderBook>,
    subscribe_instrument_sender: UnboundedSender<BitstampRequest>,
}

impl OrderBookManager {
    pub fn new(subscribe_instrument_sender: UnboundedSender<BitstampRequest>) -> Self {
        Self {
            order_books: HashMap::new(),
            subscribe_instrument_sender,
        }
    }
}

impl OrderBookManager {
    pub fn get_order_book(&mut self, symbol: &str) -> &mut OrderBook {
        let symbol = symbol.to_string();
        let order_book = self.order_books.get_mut(&symbol).unwrap();
        return order_book;
    }

    pub fn get_order_books(&self) {
        info!("Order Book = {:?}", self.order_books);
    }

    pub fn subscribe_instrument(&mut self, instrument: String, max_depth: usize) {
        self.order_books.insert(
            instrument.to_string(),
            OrderBook::new(instrument.to_string(), max_depth),
        );
        let request = BitstampRequest {
            event: BitstampRequestEvent::Subscribe,
            data: BitstampPublicChannel::live_order_book(&instrument),
        };
        self.subscribe_instrument_sender.send(request);
    }

    pub fn unsubscribe_instrument(&mut self, instrument: String) {
        let request = BitstampRequest {
            event: BitstampRequestEvent::Unsubscribe,
            data: BitstampPublicChannel::live_order_book(&instrument),
        };
        self.subscribe_instrument_sender.send(request);
        self.order_books.remove(&instrument);
        
    }

}

pub async fn process_order_book_messages(
    order_book_manager: Arc<RwLock<OrderBookManager>>,
    mut receiver: UnboundedReceiver<LiveOrderBookMessage>,
) {
    while let Some(order_message) = receiver.recv().await {
        let mut obm = order_book_manager.write().await;
        let order_book = obm.get_order_book(order_message.get_symbol());
        order_book.update_bids(order_message.get_bids());
        order_book.update_asks(order_message.get_asks());
    }
}
