mod common;
mod days;

use dotenv::dotenv;
use std::env;

use crate::days::day1 as day1;
use crate::common::logging::log;

fn main() {
    dotenv().ok();

    log("Hello World");

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    day1::test();
}
