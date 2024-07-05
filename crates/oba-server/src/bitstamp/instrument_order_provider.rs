use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::info;


const BTS_SUBSCRIBE: &str = "bts:subscribe";
const BTS_UNSUBSCRIBE: &str = "bts:unsubscribe";
const LIVE_ORDERS: &str = "live_orders";

#[derive(Debug, Serialize, Deserialize)]
pub struct BitstampInstrumentRequest {
    event: String,
    data: BitstampRequestData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitstampRequestData {
    channel: String,
}

pub struct BitstampInstrumentOrderProvider {
    url: String,
    subscribe_instrument_sender: UnboundedSender<BitstampInstrumentRequest>,
    unsubscribe_instrument_sender: UnboundedSender<BitstampInstrumentRequest>,
}

impl BitstampInstrumentOrderProvider {
    pub fn new(
        url: String,
        tx_subscriber: UnboundedSender<BitstampInstrumentRequest>,
        tx_unsubscriber: UnboundedSender<BitstampInstrumentRequest>,
    ) -> Self {
        Self {
            url,
            subscribe_instrument_sender: tx_subscriber,
            unsubscribe_instrument_sender: tx_unsubscriber,
        }
    }

    pub async fn connect(&self, mut rx_subscriber: UnboundedReceiver<BitstampInstrumentRequest>) {

        let (ws_stream, _) = connect_async(&self.url).await.expect("Unable to connect");
        let (mut write, mut read) = ws_stream.split();

        // Spawn Subscriber Instrument Task
        tokio::spawn(async move {
            while let Some(request) = rx_subscriber.recv().await {
                info!("Request = {:?}", request);
                let msg = Message::Text(
                    serde_json::to_string(&request).expect("Unable to marshal to string")
                );

                write.send(msg).await.expect("Unable to send message")
                                
            }
        });


        tokio::spawn(async move {
           while let Some(msg) = read.next().await {
               match msg {
                   Ok(message) => {
                       info!("message = {:?}", message);
                   },
                   Err(_) => {
                       panic!("Something went wrong");
                   }
               }
           } 
        });

        
    }

    pub fn subscriber_instrument(&self, instrument: String) {
        let request = BitstampInstrumentRequest {
            event: BTS_SUBSCRIBE.into(),
            data: BitstampRequestData {
                channel: format!("{}_{}", LIVE_ORDERS, instrument),
            },
        };
        self.subscribe_instrument_sender.send(request);
        
    }
}
