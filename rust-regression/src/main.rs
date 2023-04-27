extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env::args;
use std::vec::Vec;

mod linear_regression;

fn main() {
    println!("Hello, world!");
    linear_regression::run();
}
