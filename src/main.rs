use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Order {
    size: f64,
    order_type: BidOrAsk,
}

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Price {
    integral: u64,
    fractional: u64,
    scaler: u64,
}

#[derive(Debug, Clone)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

#[derive(Debug, Clone)]
enum BidOrAsk {
    Bid,
    Ask,
}

impl Order {
    fn new(size: f64, order_type: BidOrAsk) -> Order {
        Order { size, order_type }
    }
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

impl Price {
    fn new(price: f64) -> Price {
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
    fn new() -> OrderBook {
        OrderBook {
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    fn add_limit_order(&mut self, price: f64, order: Order) {
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

fn main() {
    // let price = Price::new(50.1);
    let mut order_book = OrderBook::new();
    let buy_order = Order::new(5.5, BidOrAsk::Bid);
    let sell_order = Order::new(4.0, BidOrAsk::Ask);
    let sell_order_two = Order::new(10.0, BidOrAsk::Ask);

    order_book.add_limit_order(5.5, buy_order);
    order_book.add_limit_order(4.0, sell_order);
    order_book.add_limit_order(4.0, sell_order_two);

    println!("{:?}", order_book);
}
