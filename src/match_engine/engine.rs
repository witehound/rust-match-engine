use std::collections::HashMap;

use super::orderbook::OrderBook;

#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

#[derive(Debug)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }
}
