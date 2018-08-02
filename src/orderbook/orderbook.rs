extern crate pretty_env_logger;

use orderbook::orders::Order;
use orderbook::orders::Ordertype;
use orderbook::pricelevel::PriceLevel;
use std::collections::HashMap;
use std::f64;
use serde_json;

pub struct Orderbook{
    // Describes an orderbook
    name: String, // A currency pair, ticker, whatever.
    automatch: bool, // Whether or not to match trades

    // Price levels
    buys:  HashMap<String, PriceLevel>,
    sells: HashMap<String, PriceLevel>,

    // Bests
    best_buy:  (String, f64),
    best_sell: (String, f64),
}

impl Orderbook {
    pub fn new(name: String) -> Orderbook{
        return Orderbook{
            name,
            automatch: false,
            buys: HashMap::new(),
            sells: HashMap::new(),
            best_buy:  (String::default(), f64::MIN),
            best_sell: (String::default(), f64::MAX),
        }
    }

    pub fn update_buy(&mut self, new_buy: Order, verbose: bool){
        let pl = PriceLevel::new(&new_buy);
        match self.buys.contains_key(&new_buy.price_string) {
            true  => {
                let price = new_buy.price_string;
                if verbose {
                    info!("Inserting new buy level {} at {}",
                    &price,
                    pl.quantity)
                }
                self.buys.entry(price).or_insert(pl);
            },
            false => {
                let price = new_buy.price_string;
                let entry = self.buys.entry(price.clone()).or_default();

                if new_buy.additive {
                    if verbose {
                        info!("Adjusting buy level {} by {}",
                        price,
                        pl.quantity)
                    }

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
                let price = new_sell.price_string;
                if verbose {
                    info!("Inserting new sell level {} at {}",
                    &price,
                    pl.quantity)
                }
                self.sells.entry(price).or_insert(pl);
            },
            false => {
                let price = new_sell.price_string;
                let entry = self.sells.entry(price.clone()).or_default();

                if new_sell.additive {
                    if verbose {
                        info!("Adjusting sell level {} by {}",
                        price,
                        pl.quantity)
                    }

                    (*entry).merge(pl);
                } else {
                    *entry = pl;
                }
            }
        }
    }

    pub fn insert(&mut self, new_order: Order, verbose: bool){
        self.update_best(&new_order, verbose);
        match new_order.order_type {
            Ordertype::Buy  => self.update_buy(new_order, verbose),
            Ordertype::Sell => self.update_sell(new_order, verbose)
        };
    }

    pub fn insert_from_json(&mut self, order_string: String, verbose: bool) {
        let new_order: Order = match serde_json::from_str(&order_string) {
            Ok(o) => o,
            Err(e) => {
                print!("Serializing string: {:?}", e);
                return ();
            }
        };

        if verbose {
            info!("Received JSON string:");
            info!("{}", order_string);
        }

        self.insert(new_order, verbose);
    }

    pub fn update_best(&mut self, new_order: &Order, _verbose: bool) {
        match new_order.order_type {
            Ordertype::Buy  => {
                if new_order.price > self.best_buy.1 {
                    self.best_buy = (new_order.price_string.clone(), new_order.price.clone())
                }
            },
            Ordertype::Sell => {
                if new_order.price < self.best_sell.1 {
                    self.best_sell = (new_order.price_string.clone(), new_order.price.clone())
                }
            }
        }
    }

    pub fn summary(&self) {
        let name = &self.name;
        let num_buys = self.buys.len();
        let num_sells = self.sells.len();
        let best_buy = &self.best_buy.0;
        let best_sell = &self.best_sell.0;

        println!("Name: {}
        Buy count:  {}
        Sell count: {}

        Best buy:  {}
        Best sell: {}",
        name, num_buys, num_sells, best_buy, best_sell);
    }
}
