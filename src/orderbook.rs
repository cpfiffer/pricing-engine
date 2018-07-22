use std::collections::HashMap;

pub struct Orderbook{
    // Describes an orderbook.
    buys: HashMap<Box<String>, f64>,
    sells: HashMap<Box<String>, f64>
}

impl Orderbook {
    pub fn new() -> Orderbook{
        return Orderbook{
            buys: HashMap::new(),
            sells: HashMap::new(),
        }
    }

    pub fn insert(&self, new_order: Order){
        if new_order.order_type == Ordertype::Buy {
            buys.entry(new_order.price_string).or_insert(new_order.price);
        }
        else {

        }
    }
}

enum Ordertype{
    Buy,
    Sell
}

pub struct Order {
    order_type: Ordertype,
    price_string: String,
    price: f64,
    amount: f64,
}