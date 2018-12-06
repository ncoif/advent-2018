#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle6;

fn main() {
    if let Some(day) = env::args().nth(1) {
        match day.parse::<i32>() {
            Ok(1) => puzzle1::run(),
            Ok(2) => puzzle2::run(),
            Ok(3) => puzzle3::run(),
            Ok(4) => puzzle4::run(),
            Ok(6) => puzzle6::run(),
            _ => panic!("Invalid problem day: {}", day),
        }
    } else {
        panic!("Usage: aoc <day>");
    }
}
