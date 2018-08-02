extern crate rand;
extern crate pretty_env_logger;
extern crate serde_json;

use testing::rand::ThreadRng;
use testing::rand::random;
use testing::rand::distributions::Uniform;
use testing::rand::{Rng, thread_rng};
use orderbook::orderbook::Orderbook;
use orderbook::orders::*;

pub fn run_test() {
    pretty_env_logger::init();
    let name = "testing".to_string();
    let mut ob = Orderbook::new(name);
    let cumulative = true;

    let mut rng = thread_rng();
    let price_range = Uniform::new(1, 100);
    let amount_range = Uniform::new(1, 1000000);

    for _ in 1..50 {
        let new_order = random_order(cumulative, &mut rng, &price_range, &amount_range);
        // info!("{:?}", &new_order);
        ob.insert(new_order, true);
    }

    // Test string insertion
    for _ in 1..50 {
        let new_str = random_order_string(cumulative, &mut rng, &price_range, &amount_range);
        ob.insert_from_json(new_str, true);
    }

    ob.summary();
}

pub fn random_order(cumulative: bool,
    rng: &mut ThreadRng,
    price_range: &Uniform<i32>,
    amount_range: &Uniform<i32>) -> Order {
        let price: i32 = rng.sample(price_range);
        let amount: i32 = rng.sample(amount_range);

        if random() {
            return Order::sell(price.into(), amount.into(), cumulative);
        } else {
            return Order::buy(price.into(), amount.into(), cumulative);
        }
}

pub fn random_order_string(cumulative: bool,
    rng: &mut ThreadRng,
    price_range: &Uniform<i32>,
    amount_range: &Uniform<i32>) -> String {
        let new_order = random_order(cumulative, rng, &price_range, &amount_range);
        return serde_json::to_string_pretty(&new_order).unwrap();
}
