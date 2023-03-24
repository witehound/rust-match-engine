struct Order {
    size: f64,
    order_type: BidOrAsk,
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

fn main() {
    println!("Hello, world!");
}
