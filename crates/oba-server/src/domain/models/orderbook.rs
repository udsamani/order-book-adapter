use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
pub struct LiveOrderBookMessage {
    bids: VecDeque<(f64, f64)>,
    asks: VecDeque<(f64, f64)>,
    timestamp: u64,
    symbol: String,
}

impl LiveOrderBookMessage {
    pub fn new(
        symbol: String,
        bids: VecDeque<(f64, f64)>,
        asks: VecDeque<(f64, f64)>,
        timestamp: u64,
    ) -> Self {
        Self {
            symbol,
            bids,
            asks,
            timestamp,
        }
    }
}

#[derive(Debug)]
pub struct OrderBook {
    symbol: String,
    bids: BTreeMap<u64, f64>,
    asks: BTreeMap<u64, f64>,
}

impl OrderBook {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
}
