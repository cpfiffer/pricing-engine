extern crate rand;
extern crate pretty_env_logger;

use testing::rand::prelude::random;
use orderbook::orderbook::Orderbook;
use orderbook::orders::*;

pub fn run_test() {
    pretty_env_logger::init();
    let name = "testing".to_string();
    let mut ob = Orderbook::new(name);
    let cumulative = true;

    for _ in 1..50 {
        let new_order = random_order(cumulative);
        // info!("{:?}", &new_order);
        ob.insert(new_order, true);
    }
}

pub fn random_order(cumulative: bool) -> Order {
    let price: f64 = random();
    let amount: f64 = random();

    if random() {
        return Order::sell(price - 0.5, amount, cumulative);
    } else {
        return Order::buy(price - 0.5, amount, cumulative);
    }
}
