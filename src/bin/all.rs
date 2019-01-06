use aoc::common::error::AocError;

use aoc::day01::chronal_calibration;
use aoc::day02::inventory_management_system;
use aoc::day03::no_matter_how_you_slice_it;
use aoc::day04::repose_record;
use aoc::day05::alchemical_reduction;
use aoc::day06::chronal_coordinates;
use aoc::day07::the_sum_of_its_part;
use aoc::day08::memory_maneuver;

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

        31 => println_day!(no_matter_how_you_slice_it::answer1()),
        32 => println_day!(no_matter_how_you_slice_it::answer2()),

        41 => println_day!(repose_record::answer1()),
        42 => println_day!(repose_record::answer2()),

        51 => println_day!(alchemical_reduction::answer1()),
        52 => println_day!(alchemical_reduction::answer2()),

        61 => println_day!(chronal_coordinates::answer1()),
        62 => println_day!(chronal_coordinates::answer2()),

        71 => println_day!(the_sum_of_its_part::answer1()),
        72 => println_day!(the_sum_of_its_part::answer2()),

        81 => println_day!(memory_maneuver::answer1()),
        82 => println_day!(memory_maneuver::answer2()),

        _ => println!("Error: {:?}", AocError::InvalidDayProblem),
    };
}
