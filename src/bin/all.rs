use std::env;

//mod aoc;

use aoc::day01::chronal_calibration::{answer1, answer2};

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

    let result = match day * 10 + problem {
        11 => answer1(),
        12 => answer2(),

        //TODO use error handling here
        _ => panic!("not implemented"),
    };

    match result {
        Ok(r) => println!("{}", r),
        Err(err) => println!("Error: {:?}", err),
    }
}
