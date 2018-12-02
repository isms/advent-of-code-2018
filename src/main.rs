mod day01;
mod day02;
mod utils;

use clap::{App, Arg};
use std::process::exit;

fn main() {
    let matches = App::new("Advent of Code 2018")
        .version("0.1")
        .author("Isaac Slavitt")
        .arg(Arg::with_name("day").help("Day to run").index(1))
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(d) = matches.value_of("day") {
        let d: i32 = d.parse().expect("Must be a number");
        match d {
            1 => day01::run(),
            2 => day02::run(),
            _ => {
                println!("Day not found");
                exit(1);
            }
        }
    } else {
        exit(1);
    }
}
