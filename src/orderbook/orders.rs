extern crate serde_json;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Ordertype{
    Buy = 1,
    Sell = -1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub order_type: Ordertype,
    pub price_string: String,
    pub price: f64,
    pub amount: f64,
    pub additive: bool
}

impl Order {
    pub fn sell(price: f64, amount: f64, additive: bool) -> Order {
        return Order {
            order_type: Ordertype::Sell,
            price_string: price.to_string(),
            price: price,
            amount,
            additive
        }
    }

    pub fn buy(price: f64, amount: f64, additive: bool) -> Order {
        return Order {
            order_type: Ordertype::Buy,
            price_string: price.to_string(),
            price: price,
            amount,
            additive
        }
    }
}
