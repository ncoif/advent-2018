use std::env;

mod puzzle1;

fn main() {
    if let Some(day) = env::args().nth(1) {
        match day.parse::<i32>() {
            Ok(1) => puzzle1::run(),
            _ => panic!("Invalid problem day: {}", day),
        }
    } else {
        panic!("Usage: aoc <day>");
    }
}