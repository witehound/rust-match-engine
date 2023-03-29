mod match_engine;
use match_engine::{engine::*, orderbook::*};
fn main() {
    // let price = Price::new(50.1);
    let mut order_book = OrderBook::new();
    let buy_order = Order::new(5.5, BidOrAsk::Bid);
    let sell_order = Order::new(4.0, BidOrAsk::Ask);
    let sell_order_two = Order::new(10.0, BidOrAsk::Ask);

    order_book.add_limit_order(5.5, buy_order);
    order_book.add_limit_order(4.0, sell_order);
    order_book.add_limit_order(4.0, sell_order_two);

    let engine = MatchingEngine::new();

    println!("{:?}", order_book);
}
