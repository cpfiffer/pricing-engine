use orderbook::orders::Order;
use std::ops::Add;

#[derive(Default)]
pub struct PriceLevel {
    pub quantity: f64
}

impl PriceLevel {
    pub fn new(order: &Order) -> PriceLevel {
        return PriceLevel {quantity: order.amount}
    }

    pub fn merge(&self, other: PriceLevel) {
        self.quantity += other.quantity;
    }
}
