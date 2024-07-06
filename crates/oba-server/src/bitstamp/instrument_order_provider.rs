use std::sync::Arc;

use futures_util::{SinkExt as _, StreamExt};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{error, info};

use crate::domain::models::orderbook::LiveOrderBookMessage;

use super::models::{
    bitstamp_response_to_live_order_message, BitstampRequest,BitstampResponse,
};

pub struct BitstampInstrumentOrderProvider {
    url: String,
}

impl BitstampInstrumentOrderProvider {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn connect(
        &self,
        mut rx_subscriber: UnboundedReceiver<BitstampRequest>,
        tx_order_message_sender: UnboundedSender<LiveOrderBookMessage>,
    ) {
        let (ws_stream, _) = connect_async(&self.url).await.expect("Unable to connect");
        let (write, mut read) = ws_stream.split();

        let write = Arc::new(Mutex::new(write));

        // Spawn Subscriber Instrument Task
        let write_request = write.clone();
        tokio::spawn(async move {
            while let Some(request) = rx_subscriber.recv().await {
                info!("Request = {:?}", request);
                let msg = Message::Text(
                    serde_json::to_string(&request).expect("Unable to marshal to string"),
                );

                let mut w = write_request.lock().await;
                w.send(msg).await.expect("Unable to send message")
            }
        });

        let write_pong = write.clone();
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                // info!("{:?}", msg);
                match msg {
                    Ok(Message::Text(message)) => {
                        let bitstamp_response =
                            serde_json::from_str::<BitstampResponse>(&message).unwrap();
                        match bitstamp_response {
                            BitstampResponse::SubscriptionSucceeded { channel } => {
                                info!("Successfully subscribed {}", channel);
                            }
                            BitstampResponse::LiveOrderBookData { data, channel } => {
                                let order_message: LiveOrderBookMessage =
                                    bitstamp_response_to_live_order_message(data, &channel);

                                match tx_order_message_sender.send(order_message) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        error!("error while sending message to order book manager {}", err);
                                    },
                                }
                            }
                            BitstampResponse::UnsubscriptionSucceeded { channel } => {
                                info!("Successfully unsubscribed {}", channel);
                            },
                        }
                    }
                    Err(e) => {
                        info!("{}", e);
                    }
                    Ok(Message::Ping(ping)) => {
                        info!("Ping message received = {:?}", ping);
                        let mut w = write_pong.lock().await;
                        w.send(Message::Pong(ping))
                            .await
                            .expect("Unable to send PONG message");
                    }
                    _ => panic!("Unknown message from websocket"),
                }
            }
        });
    }
}
