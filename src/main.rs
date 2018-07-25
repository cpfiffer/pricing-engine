#[macro_use]
extern crate log;

pub mod orderbook;
pub mod testing;

fn main(){
    testing::run_test();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
