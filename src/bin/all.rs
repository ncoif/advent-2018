#![feature(test)]
extern crate test;

mod common;
mod day01;

fn main() {
    match day01::chronal_calibration::answer1() {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    match day01::chronal_calibration::answer2() {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
