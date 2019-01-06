#[macro_use]
extern crate criterion;

use aoc::day01::chronal_calibration;
use aoc::day02::inventory_management_system;
use aoc::day03::no_matter_how_you_slice_it;
use aoc::day04::repose_record;
use aoc::day05::alchemical_reduction;
use aoc::day06::chronal_coordinates;
use aoc::day07::the_sum_of_its_part;
use aoc::day08::memory_maneuver;
use aoc::day09::marble_mania;
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

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 01 answer 1", |b| {
        b.iter(|| chronal_calibration::answer1())
    });
    c.bench_function("day 01 answer 2", |b| {
        b.iter(|| chronal_calibration::answer2())
    });

    c.bench_function("day 02 answer 1", |b| {
        b.iter(|| inventory_management_system::answer1())
    });
    c.bench_function("day 02 answer 2", |b| {
        b.iter(|| inventory_management_system::answer2())
    });

    c.bench_function("day 03 answer 1", |b| {
        b.iter(|| no_matter_how_you_slice_it::answer1())
    });
    c.bench_function("day 03 answer 2", |b| {
        b.iter(|| no_matter_how_you_slice_it::answer2())
    });

    c.bench_function("day 04 answer 1", |b| b.iter(|| repose_record::answer1()));
    c.bench_function("day 04 answer 2", |b| b.iter(|| repose_record::answer2()));

    c.bench_function("day 05 answer 1", |b| {
        b.iter(|| alchemical_reduction::answer1())
    });
    c.bench_function("day 05 answer 2", |b| {
        b.iter(|| alchemical_reduction::answer2())
    });

    c.bench_function("day 06 answer 1", |b| {
        b.iter(|| chronal_coordinates::answer1())
    });
    c.bench_function("day 06 answer 2", |b| {
        b.iter(|| chronal_coordinates::answer2())
    });

    c.bench_function("day 07 answer 1", |b| {
        b.iter(|| the_sum_of_its_part::answer1())
    });
    c.bench_function("day 07 answer 2", |b| {
        b.iter(|| the_sum_of_its_part::answer2())
    });

    c.bench_function("day 08 answer 1", |b| b.iter(|| memory_maneuver::answer1()));
    c.bench_function("day 08 answer 2", |b| b.iter(|| memory_maneuver::answer2()));

    c.bench_function("day 09 answer 1", |b| b.iter(|| marble_mania::answer1()));
    c.bench_function("day 09 answer 2", |b| b.iter(|| marble_mania::answer2()));

    c.bench_function("day 11 answer 1", |b| b.iter(|| chronal_charge::answer1()));
    c.bench_function("day 11 answer 2", |b| b.iter(|| chronal_charge::answer2()));

    c.bench_function("day 12 answer 1", |b| {
        b.iter(|| subterranean_sustainability::answer1())
    });
    c.bench_function("day 12 answer 2", |b| {
        b.iter(|| subterranean_sustainability::answer2())
    });

    c.bench_function("day 13 answer 1", |b| {
        b.iter(|| mine_cart_madness::answer1())
    });
    c.bench_function("day 13 answer 2", |b| {
        b.iter(|| mine_cart_madness::answer2())
    });

    c.bench_function("day 14 answer 1", |b| {
        b.iter(|| chocolate_charts::answer1())
    });
    c.bench_function("day 14 answer 2", |b| {
        b.iter(|| chocolate_charts::answer2())
    });

    c.bench_function("day 15 answer 1", |b| {
        b.iter(|| beverage_bandits::answer1())
    });
    c.bench_function("day 15 answer 2", |b| {
        b.iter(|| beverage_bandits::answer2())
    });

    c.bench_function("day 16 answer 1", |b| {
        b.iter(|| chronal_classification::answer1())
    });
    c.bench_function("day 16 answer 2", |b| {
        b.iter(|| chronal_classification::answer2())
    });

    c.bench_function("day 17 answer 1", |b| {
        b.iter(|| reservoir_research::answer1())
    });
    c.bench_function("day 17 answer 2", |b| {
        b.iter(|| reservoir_research::answer2())
    });

    c.bench_function("day 18 answer 1", |b| {
        b.iter(|| settlers_of_the_north_pole::answer1())
    });
    c.bench_function("day 18 answer 2", |b| {
        b.iter(|| settlers_of_the_north_pole::answer2())
    });

    c.bench_function("day 19 answer 1", |b| {
        b.iter(|| go_with_the_flow::answer1())
    });
    c.bench_function("day 19 answer 2", |b| {
        b.iter(|| go_with_the_flow::answer2())
    });

    c.bench_function("day 20 answer 1", |b| b.iter(|| a_regular_map::answer1()));
    c.bench_function("day 20 answer 2", |b| b.iter(|| a_regular_map::answer2()));

    c.bench_function("day 21 answer 1", |b| {
        b.iter(|| chronal_conversion::answer1())
    });
    c.bench_function("day 21 answer 2", |b| {
        b.iter(|| chronal_conversion::answer2())
    });

    c.bench_function("day 22 answer 1", |b| b.iter(|| mode_maze::answer1()));
    c.bench_function("day 22 answer 2", |b| b.iter(|| mode_maze::answer2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
