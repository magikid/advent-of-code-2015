extern crate env_logger;
#[macro_use]
extern crate log;

use env_logger::Env;
use std::env;

mod day1;
mod day2;
mod day3;

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    debug!("Printing answer for day {:?}", day.as_str());

    match day.as_str() {
        "1" => println!("day1 p1: {:?}, p2: {:?}", day1::p1(), day1::p2()),
        "2" => println!("day2 p1: {:?}, p2: {:?}", day2::p1(), day2::p2()),
        "3" => println!("day3 p1: {:?}, p2: {:?}", day3::p1(), day3::p2()),
        _ => panic!("You forgot to specify a day"),
    }
}
