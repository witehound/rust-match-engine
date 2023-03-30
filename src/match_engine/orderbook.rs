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

    pub fn is_filled(&mut self) -> bool {
        self.size == 0.0
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.size)
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

    pub fn fill_order(&mut self, order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match order.size >= limit_order.size {
                true => {
                    order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }
                false => {
                    limit_order.size -= order.size;
                    order.size = 0.0;
                }
            }

            if order.is_filled() {
                break;
            }
        }
    }

    pub fn total_volume(&mut self) -> f64 {
        let mut size: f64 = 0.0;
        for limit_order in self.orders.iter_mut() {
            size += limit_order.size;
        }
        size
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

    pub fn fill_market_order(&mut self, price: f64, order: Order) -> Result<(), f64> {
        let price = Price::new(price);
        match order.order_type {
            BidOrAsk::Ask => {
                let limit = self.asks.get_mut(&price);
                match limit {
                    Some(limit) => {
                        if limit.total_volume() < order.size {
                            Err(format!("Not enough volume to match order {:?}", order.size))
                        }

                        Ok(())
                    }
                    None => Err(format!("price out of ranges")),
                }
            }
            BidOrAsk::Bid => {
                let limit = self.asks.get_mut(&price);
                match limit {
                    Some(limit) => {
                        if limit.total_volume() < order.size {
                            Err(format!("Not enough volume to match order"))
                        }

                        Ok(())
                    }
                    None => Err(format!("price out of ranges")),
                }
            }
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn limit_order_single_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new(100.0, BidOrAsk::Bid);

        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(96.0, BidOrAsk::Ask);

        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 4.0);
    }

    #[test]
    fn limit_order_multiple_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(100.0, BidOrAsk::Bid);
        let buy_limit_order_b = Order::new(155.0, BidOrAsk::Bid);

        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_sell_order_a = Order::new(50.0, BidOrAsk::Ask);

        let mut market_sell_order_b = Order::new(56.0, BidOrAsk::Ask);

        limit.fill_order(&mut market_sell_order_a);

        limit.fill_order(&mut market_sell_order_b);

        assert_eq!(market_sell_order_a.is_filled(), true);
        assert_eq!(market_sell_order_b.is_filled(), true);

        assert_eq!(limit.orders.get(1).unwrap().size, 149.0);
    }

    #[test]
    fn total_limit() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(100.0, BidOrAsk::Bid);
        let buy_limit_order_b = Order::new(155.0, BidOrAsk::Bid);

        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 255.0);
    }
}
