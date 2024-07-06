use axum::http::response;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::info;

use crate::domain::models::orderbook::LiveOrderBookMessage;

use super::models::{bitstamp_response_to_live_order_message, BitstampPublicChannel, BitstampRequest, BitstampRequestEvent, BitstampResponse};



pub struct BitstampInstrumentOrderProvider {
    url: String,
    subscribe_instrument_sender: UnboundedSender<BitstampRequest>,
    unsubscribe_instrument_sender: UnboundedSender<BitstampRequest>,
}

impl BitstampInstrumentOrderProvider {
    pub fn new(
        url: String,
        tx_subscriber: UnboundedSender<BitstampRequest>,
        tx_unsubscriber: UnboundedSender<BitstampRequest>,
    ) -> Self {
        Self {
            url,
            subscribe_instrument_sender: tx_subscriber,
            unsubscribe_instrument_sender: tx_unsubscriber,
        }
    }

    pub async fn connect(
        &self,
        mut rx_subscriber: UnboundedReceiver<BitstampRequest>,
        tx_order_message_sender: UnboundedSender<LiveOrderBookMessage>,
    ) {
        let (ws_stream, _) = connect_async(&self.url).await.expect("Unable to connect");
        let (mut write, mut read) = ws_stream.split();

        // Spawn Subscriber Instrument Task
        tokio::spawn(async move {
            while let Some(request) = rx_subscriber.recv().await {
                info!("Request = {:?}", request);
                let msg = Message::Text(
                    serde_json::to_string(&request).expect("Unable to marshal to string"),
                );

                write.send(msg).await.expect("Unable to send message")
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(message)) => {
                        let bitstamp_response = serde_json::from_str::<BitstampResponse>(&message).unwrap();
                        match bitstamp_response{
                            BitstampResponse::SubscriptionSucceeded { channel } => {
                                info!("Successfully subscribed {}", channel);
                            }
                            BitstampResponse::LiveOrderBookData { data, channel } => {
                                let order_message: LiveOrderBookMessage = bitstamp_response_to_live_order_message(data, channel);
                                tx_order_message_sender.send(order_message);
                            },
                        }
                    }
                    Err(_) => {
                        panic!("Something went wrong");
                    }
                    _ => panic!("Unknown value"),
                }
            }
        });
    }

    pub fn subscriber_instrument(&self, instrument: String) {
        let request = BitstampRequest{
            event: BitstampRequestEvent::Subscribe,
            data: BitstampPublicChannel::live_order_book(instrument)
        };
        self.subscribe_instrument_sender.send(request);
    }
}
