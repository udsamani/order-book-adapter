use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    RwLock,
};
use tracing::{error, info};

use crate::bitstamp::models::{BitstampPublicChannel, BitstampRequest, BitstampRequestEvent};

use super::models::orderbook::{LiveOrderBookMessage, OrderBook};

#[derive(Debug)]
pub struct OrderBookManager {
    order_books: HashMap<String, OrderBook>,
    instrument_request_sender: UnboundedSender<BitstampRequest>,
}

impl OrderBookManager {
    pub fn new(subscribe_instrument_sender: UnboundedSender<BitstampRequest>) -> Self {
        Self {
            order_books: HashMap::new(),
            instrument_request_sender: subscribe_instrument_sender,
        }
    }
}

impl OrderBookManager {
    pub fn get_mut_order_book(&mut self, symbol: &str) -> Option<&mut OrderBook> {
        let symbol = symbol.to_string();
        return self.order_books.get_mut(&symbol);
    }

    pub fn get_order_book(&self, instrument: &str) -> Option<&OrderBook> {
        self.order_books.get(instrument)
    }

    pub fn subscribe_instrument(&mut self, instrument: String, max_depth: usize) {
        self.order_books.insert(
            instrument.to_string(),
            OrderBook::new(max_depth),
        );
        let request = BitstampRequest {
            event: BitstampRequestEvent::Subscribe,
            data: BitstampPublicChannel::live_order_book(&instrument),
        };
        match self.instrument_request_sender.send(request) {
            Ok(_) => info!("successfully sent message to subscribe instrument {}", &instrument),
            Err(e) => error!("unable to send message to subscribe instrument {}, error {}", &instrument, e)
        }
    }

    pub fn unsubscribe_instrument(&mut self, instrument: String) {
        let request = BitstampRequest {
            event: BitstampRequestEvent::Unsubscribe,
            data: BitstampPublicChannel::live_order_book(&instrument),
        };
        self.instrument_request_sender.send(request);
        self.order_books.remove(&instrument);
        
    }

}

pub async fn process_order_book_messages(
    order_book_manager: Arc<RwLock<OrderBookManager>>,
    mut receiver: UnboundedReceiver<LiveOrderBookMessage>,
) {
    while let Some(order_message) = receiver.recv().await {
        let mut obm = order_book_manager.write().await;
        let order_book = obm.get_mut_order_book(order_message.get_symbol());
        match order_book {
            Some(ob) => {
                ob.update_bids(order_message.get_bids());
                ob.update_asks(order_message.get_asks());
            },
            None => {
                tracing::error!("Could not find order book, maybe it was unsubscribed");
            }
        }
    }
}
