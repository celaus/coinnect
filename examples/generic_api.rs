// This example shows how to use the generic API provided by coinnect
// This method is useful if you have to iterate throught multiple accounts of
// different exchanges and perform the same operation (such as get the current account's balance)

extern crate coinnect;

use coinnect::coinnect::Coinnect;
use coinnect::exchange::Exchange::*;

fn main() {
    // We create a Coinnect Generic API
    // as Poloniex does not need customer_id field, we keep it empty
    let mut _my_api = Coinnect::new(Poloniex, "", "api_key", "api_secret");

}