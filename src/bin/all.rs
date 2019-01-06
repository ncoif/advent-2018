use aoc::common::error::AocError;
use aoc::day01::chronal_calibration::{answer1, answer2};
use std::env;

fn main() {
    let args = read_argument();
    if args.is_err() {
        println!("Error: {:?}", args.err().unwrap());
        return;
    }
    let (day, problem) = args.unwrap();

    let result = match day * 10 + problem {
        11 => answer1(),
        12 => answer2(),

        _ => Err(AocError::InvalidDayProblem),
    };

    match result {
        Ok(r) => println!("{}", r),
        Err(err) => println!("Error: {:?}", err),
    }
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
