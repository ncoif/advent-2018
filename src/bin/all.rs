use aoc::common::error::AocError;
use aoc::day01::chronal_calibration;
use aoc::day02::inventory_management_system;

use std::env;

macro_rules! println_day {
    ($e:expr) => {
        match $e {
            Ok(r) => println!("{}", r),
            Err(err) => println!("Error: {:?}", err),
        }
    };
}

macro_rules! try_opt {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return Err(AocError::InvalidDayProblem),
        }
    };
}

fn read_argument() -> Result<(u32, u32), AocError> {
    let day = try_opt!(env::args().nth(1));
    let day = day.parse::<u32>()?;

    let problem = try_opt!(env::args().nth(2));
    let problem = problem.parse::<u32>()?;

    Ok((day, problem))
}

fn main() {
    let args = read_argument();
    if args.is_err() {
        println!("Error: {:?}", args.err().unwrap());
        return;
    }
    let (day, problem) = args.unwrap();

    match day * 10 + problem {
        11 => println_day!(chronal_calibration::answer1()),
        12 => println_day!(chronal_calibration::answer2()),

        21 => println_day!(inventory_management_system::answer1()),
        22 => println_day!(inventory_management_system::answer2()),

        _ => println!("Error: {:?}", AocError::InvalidDayProblem),
    };
}
