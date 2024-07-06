use serde::de::Error as DeError;
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::VecDeque;

use crate::domain::models::orderbook::LiveOrderBookMessage;

const LIVE_ORDER_BOOK: &str = "order_book";

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum BitstampResponse {
    #[serde(rename = "bts:subscription_succeeded")]
    SubscriptionSucceeded { channel: String },
    #[serde(rename = "data")]
    LiveOrderBookData {
        data: BitstampLiveOrderBookData,
        channel: String,
    },
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct BitstampLiveOrderBookData {
    #[serde_as(as = "DisplayFromStr")]
    timestamp: u64,
    #[serde(deserialize_with = "deserialize_bids_or_asks")]
    bids: VecDeque<(f64, f64)>,
    #[serde(deserialize_with = "deserialize_bids_or_asks")]
    asks: VecDeque<(f64, f64)>,
}

#[derive(Debug, Serialize)]
pub enum BitstampRequestEvent {
    #[serde(rename = "bts:subscribe")]
    Subscribe,
    #[serde(rename = "bts:unsubscribe")]
    Unsubscribe,
}

#[derive(Debug, Serialize)]
pub struct BitstampRequest {
    pub event: BitstampRequestEvent,
    pub data: BitstampPublicChannel,
}

#[derive(Debug)]
pub enum BitstampPublicChannel {
    LiverOrderBook { channel: String },
}

pub fn bitstamp_response_to_live_order_message(
    data: BitstampLiveOrderBookData,
    channel: String,
) -> LiveOrderBookMessage {
    return LiveOrderBookMessage::new(channel, data.bids, data.asks, data.timestamp);
}

impl BitstampPublicChannel {
    pub fn live_order_book(instrument: String) -> Self {
        Self::LiverOrderBook {
            channel: format!("{}_{}", LIVE_ORDER_BOOK, instrument),
        }
    }
}

impl<'a> Serialize for BitstampPublicChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BitstampPublicChannel", 1)?;
        match self {
            BitstampPublicChannel::LiverOrderBook { channel } => {
                state.serialize_field("channel", channel)?
            }
        }
        state.end()
    }
}

pub fn deserialize_bids_or_asks<'de, D>(deserializer: D) -> Result<VecDeque<(f64, f64)>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_orders: Vec<Vec<String>> = Deserialize::deserialize(deserializer)?;
    let mut orders = VecDeque::with_capacity(raw_orders.len());

    for raw_order in raw_orders {
        let price = raw_order[0].parse::<f64>().map_err(DeError::custom)?;
        let amount = raw_order[1].parse::<f64>().map_err(DeError::custom)?;
        orders.push_back((price, amount));
    }
    Ok(orders)
}
