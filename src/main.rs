struct Order {
    size: f64,
    order_type: BidOrAsk,
}

struct Price {
    integral: u64,
    fractional: u64,
    scaler: u64,
}

struct Limit {
    price: Price,
    oredr: Vec<Order>,
}

enum BidOrAsk {
    Bid,
    Ask,
}

impl Order {
    fn new(size: f64, order_type: BidOrAsk) -> Order {
        Order { size, order_type }
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
    println!("Hello, world!");
}
