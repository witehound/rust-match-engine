#[derive(Debug)]
struct Order {
    size: f64,
    order_type: BidOrAsk,
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
    order: Vec<Order>,
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
            order: Vec::new(),
        }
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
    let limit = Limit::new(50.1);
    println!("{:?}", limit);
}
