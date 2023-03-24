use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Order {
    size: f64,
    order_type: BidOrAsk,
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scaler: u64,
}

#[derive(Debug, Clone)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

#[derive(Debug, Clone)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

impl Order {
    pub fn new(size: f64, order_type: BidOrAsk) -> Order {
        Order { size, order_type }
    }
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scaler: u64 = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scaler as f64) as u64;
        Price {
            integral,
            fractional,
            scaler,
        }
    }
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    pub fn add_limit_order(&mut self, price: f64, order: Order) {
        match order.order_type {
            BidOrAsk::Ask => {
                let price = Price::new(price);
                let limit = self.asks.get_mut(&price);
                match limit {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut new_limit = Limit::new(price);
                        new_limit.add_order(order);
                        self.asks.insert(price, new_limit);
                    }
                }
            }
            BidOrAsk::Bid => {
                let price = Price::new(price);
                let limit = self.bids.get_mut(&price);
                match limit {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut new_limit = Limit::new(price);
                        new_limit.add_order(order);
                        self.bids.insert(price, new_limit);
                    }
                }
            }
        }
    }
}
