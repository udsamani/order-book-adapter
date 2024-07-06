use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
pub struct LiveOrderBookMessage {
    bids: VecDeque<(u64, f64)>,
    asks: VecDeque<(u64, f64)>,
    timestamp: u64,
    symbol: String,
}

impl LiveOrderBookMessage {
    pub fn new(
        symbol: String,
        bids: VecDeque<(u64, f64)>,
        asks: VecDeque<(u64, f64)>,
        timestamp: u64,
    ) -> Self {
        Self {
            symbol,
            bids,
            asks,
            timestamp,
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_bids(&self) -> &VecDeque<(u64, f64)> {
        &self.bids
    }

    pub fn get_asks(&self) -> &VecDeque<(u64, f64)> {
        &self.asks
    }
}

#[derive(Debug)]
pub struct OrderBook {
    symbol: String,
    bids: BTreeMap<u64, f64>,
    asks: BTreeMap<u64, f64>,
    max_depth: usize,
}

impl OrderBook {
    pub fn new(symbol: String, max_depth: usize) -> Self {
        Self {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            max_depth,
        }
    }

    pub fn update_bids(&mut self, bids: &VecDeque<(u64, f64)>) {
        for (price, amount) in bids {
            let price = price.clone();
            let amount = amount.clone();
            if self.bids.len() + 1 > self.max_depth {
                if let Some(&lowest_price) = self.bids.keys().next() {
                    self.bids.remove(&lowest_price);
                }
            }
            self.bids
                .entry(price)
                .and_modify(|a| *a += amount)
                .or_insert(amount);
        }
    }

    pub fn update_asks(&mut self, asks: &VecDeque<(u64, f64)>) {
        for (price, amount) in asks {
            let price = price.clone();
            let amount = amount.clone();
            if self.asks.len() + 1 > self.max_depth {
                if let Some(&highest_price) = self.asks.keys().next_back() {
                    self.asks.remove(&highest_price);
                }
            }
            self.asks
                .entry(price)
                .and_modify(|a| *a += amount)
                .or_insert(amount);
        }
    }
}
