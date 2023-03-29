use std::collections::HashMap;

use super::orderbook::*;

#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}", self.base, self.quote)
    }
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("Opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: f64, order: Order) {
        let market = self.orderbooks.get_mut(&pair);
        match market {
            Some(market) => market.add_limit_order(price, order),
            None => println!("{:?} market is yet to be added", pair.to_string()),
        }
    }
}
