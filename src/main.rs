use std::collections::HashMap;

#[derive(Debug)]
struct Order {
    size: f64,
    order_type: BidOrAsk,
}

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    buys: HashMap<Price, Limit>,
}

#[derive(Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scaler: u64,
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

#[derive(Debug)]
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
    fn new(price: f64) -> Limit {
        Limit {
            price: Price::new(price),
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

fn main() {
    // let price = Price::new(50.1);
    let mut limit = Limit::new(63.45);
    let buy_order = Order::new(5.5, BidOrAsk::Bid);
    let sell_order = Order::new(4.0, BidOrAsk::Ask);
    limit.add_order(buy_order);
    limit.add_order(sell_order);
    println!("{:?}", limit);
}
