use aoc::common::error::AocError;

use aoc::day01::chronal_calibration;
use aoc::day02::inventory_management_system;
use aoc::day03::no_matter_how_you_slice_it;
use aoc::day04::repose_record;
use aoc::day05::alchemical_reduction;
use aoc::day06::chronal_coordinates;
use aoc::day07::the_sum_of_its_part;
use aoc::day08::memory_maneuver;
use aoc::day09::marble_mania;
use aoc::day10::the_stars_align;
use aoc::day11::chronal_charge;
use aoc::day12::subterranean_sustainability;
use aoc::day13::mine_cart_madness;
use aoc::day14::chocolate_charts;
use aoc::day15::beverage_bandits;
use aoc::day16::chronal_classification;
use aoc::day17::reservoir_research;
use aoc::day18::settlers_of_the_north_pole;
use aoc::day19::go_with_the_flow;
use aoc::day20::a_regular_map;
use aoc::day21::chronal_conversion;
use aoc::day22::mode_maze;
use aoc::day23::experimental_emergency_teleportation;

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

        91 => println_day!(marble_mania::answer1()),
        92 => println_day!(marble_mania::answer2()),

        101 => println_day!(the_stars_align::answer1()),

        111 => println_day!(chronal_charge::answer1()),
        112 => println_day!(chronal_charge::answer2()),

        121 => println_day!(subterranean_sustainability::answer1()),
        122 => println_day!(subterranean_sustainability::answer2()),

        131 => println_day!(mine_cart_madness::answer1()),
        132 => println_day!(mine_cart_madness::answer2()),

        141 => println_day!(chocolate_charts::answer1()),
        142 => println_day!(chocolate_charts::answer2()),

        151 => println_day!(beverage_bandits::answer1()),
        152 => println_day!(beverage_bandits::answer2()),

        161 => println_day!(chronal_classification::answer1()),
        162 => println_day!(chronal_classification::answer2()),

        171 => println_day!(reservoir_research::answer1()),
        172 => println_day!(reservoir_research::answer2()),

        181 => println_day!(settlers_of_the_north_pole::answer1()),
        182 => println_day!(settlers_of_the_north_pole::answer2()),

        191 => println_day!(go_with_the_flow::answer1()),
        192 => println_day!(go_with_the_flow::answer2()),

        201 => println_day!(a_regular_map::answer1()),
        202 => println_day!(a_regular_map::answer2()),

        211 => println_day!(chronal_conversion::answer1()),
        212 => println_day!(chronal_conversion::answer2()),

        221 => println_day!(mode_maze::answer1()),
        222 => println_day!(mode_maze::answer2()),

        231 => println_day!(experimental_emergency_teleportation::answer1()),

        _ => println!("Error: {:?}", AocError::InvalidDayProblem),
    };
}
