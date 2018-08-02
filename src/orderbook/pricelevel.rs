use orderbook::orders::Order;

#[derive(Default)]
pub struct PriceLevel {
    pub quantity: f64
}

impl PriceLevel {
    pub fn new(order: &Order) -> PriceLevel {
        return PriceLevel {quantity: order.amount}
    }

    pub fn merge(&mut self, other: PriceLevel) {
        self.quantity += other.quantity;
    }
}
