extern crate pretty_env_logger;
use orderbook::orders::Order;
use orderbook::orders::Ordertype;
use orderbook::pricelevel::PriceLevel;
use std::collections::HashMap;

pub struct Orderbook{
    // Describes an orderbook.
    name: String, // A currency pair, ticker, whatever.

    // Price levels.
    buys:  HashMap<String, PriceLevel>,
    sells: HashMap<String, PriceLevel>
}

impl Orderbook {
    pub fn new(name: String) -> Orderbook{
        return Orderbook{
            name,
            buys: HashMap::new(),
            sells: HashMap::new(),
        }
    }

    pub fn update_buy(&mut self, new_buy: Order, verbose: bool){
        let pl = PriceLevel::new(&new_buy);
        match self.buys.contains_key(&new_buy.price_string) {
            true  => {
                self.buys.entry(new_buy.price_string).or_insert(pl);
            },
            false => {
                let entry = self.buys.entry(new_buy.price_string).or_default();

                if new_buy.additive {
                    (*entry).merge(pl);
                } else {
                    *entry = pl;
                }
            }
        }
    }

    pub fn update_sell(&mut self, new_sell: Order, verbose: bool){
        let pl = PriceLevel::new(&new_sell);
        match self.sells.contains_key(&new_sell.price_string) {
            true  => {
                self.sells.entry(new_sell.price_string).or_insert(pl);
            },
            false => {
                let entry = self.sells.entry(new_sell.price_string).or_default();
                *entry = pl;
            }
        }
    }

    pub fn insert(&mut self, new_order: Order, verbose: bool){
        match new_order.order_type {
            Ordertype::Buy => self.update_buy(new_order, verbose),
            Ordertype::Sell => self.update_sell(new_order, verbose)
        };
    }
}
