mod common;
mod days;


use crate::days::day1 as day1;
use crate::common::logging::log;

fn main() {
    log("Hello World");

    day1::test();
}
