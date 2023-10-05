mod days;
mod lib;

use crate::days::day1 as day1;
use crate::lib::logging::log;

fn main() {
    log("Hello World");

    day1::test();
}
