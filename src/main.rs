#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;

fn main() {
    let day: u32 = env::args()
        .nth(1)
        .expect("missing day")
        .parse::<u32>()
        .expect("invalid day");

    let problem: u32 = env::args()
        .nth(2)
        .expect("missing problem")
        .parse::<u32>()
        .expect("invalid problem");

    match day * 10 + problem {
        11 => puzzle1::answer1(),
        12 => puzzle1::answer2(),

        21 => puzzle2::answer1(),
        22 => puzzle2::answer2(),

        31 => puzzle3::answer1(),
        32 => puzzle3::answer2(),

        41 => puzzle4::answer1(),
        42 => puzzle4::answer2(),

        51 => puzzle5::answer1(),
        52 => puzzle5::answer2(),

        61 => puzzle6::answer1(),
        62 => puzzle6::answer2(),

        _ => panic!("Invalid problem day: {}, {}", day, problem),
    }
}
