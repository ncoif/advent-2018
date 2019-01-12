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
use aoc::day23::experimental_emergency_teleportation;
use aoc::day24::immune_system_simulator;
use aoc::day25::four_dimensional_adventure;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d01_a1", |b| b.iter(|| chronal_calibration::answer1()));
    c.bench_function("d01_a2", |b| b.iter(|| chronal_calibration::answer2()));

    c.bench_function("d02_a1", |b| {
        b.iter(|| inventory_management_system::answer1())
    });
    c.bench_function("d02_a2", |b| {
        b.iter(|| inventory_management_system::answer2())
    });

    c.bench_function("d03_a1", |b| {
        b.iter(|| no_matter_how_you_slice_it::answer1())
    });
    c.bench_function("d03_a2", |b| {
        b.iter(|| no_matter_how_you_slice_it::answer2())
    });

    c.bench_function("d04_a1", |b| b.iter(|| repose_record::answer1()));
    c.bench_function("d04_a2", |b| b.iter(|| repose_record::answer2()));

    c.bench_function("d05_a1", |b| b.iter(|| alchemical_reduction::answer1()));
    c.bench_function("d05_a2", |b| b.iter(|| alchemical_reduction::answer2()));

    c.bench_function("d06_a1", |b| b.iter(|| chronal_coordinates::answer1()));
    c.bench_function("d06_a2", |b| b.iter(|| chronal_coordinates::answer2()));

    c.bench_function("d07_a1", |b| b.iter(|| the_sum_of_its_part::answer1()));
    c.bench_function("d07_a2", |b| b.iter(|| the_sum_of_its_part::answer2()));

    c.bench_function("d08_a1", |b| b.iter(|| memory_maneuver::answer1()));
    c.bench_function("d08_a2", |b| b.iter(|| memory_maneuver::answer2()));

    c.bench_function("d09_a1", |b| b.iter(|| marble_mania::answer1()));
    c.bench_function("d09_a2", |b| b.iter(|| marble_mania::answer2()));

    c.bench_function("d11_a1", |b| b.iter(|| chronal_charge::answer1()));
    c.bench_function("d11_a2", |b| b.iter(|| chronal_charge::answer2()));

    c.bench_function("d12_a1", |b| {
        b.iter(|| subterranean_sustainability::answer1())
    });
    c.bench_function("d12_a2", |b| {
        b.iter(|| subterranean_sustainability::answer2())
    });

    c.bench_function("d13_a1", |b| b.iter(|| mine_cart_madness::answer1()));
    c.bench_function("d13_a2", |b| b.iter(|| mine_cart_madness::answer2()));

    c.bench_function("d14_a1", |b| b.iter(|| chocolate_charts::answer1()));
    c.bench_function("d14_a2", |b| b.iter(|| chocolate_charts::answer2()));

    c.bench_function("d15_a1", |b| b.iter(|| beverage_bandits::answer1()));
    c.bench_function("d15_a2", |b| b.iter(|| beverage_bandits::answer2()));

    c.bench_function("d16_a1", |b| b.iter(|| chronal_classification::answer1()));
    c.bench_function("d16_a2", |b| b.iter(|| chronal_classification::answer2()));

    c.bench_function("d17_a1", |b| b.iter(|| reservoir_research::answer1()));
    c.bench_function("d17_a2", |b| b.iter(|| reservoir_research::answer2()));

    c.bench_function("d18_a1", |b| {
        b.iter(|| settlers_of_the_north_pole::answer1())
    });
    c.bench_function("d18_a2", |b| {
        b.iter(|| settlers_of_the_north_pole::answer2())
    });

    c.bench_function("d19_a1", |b| b.iter(|| go_with_the_flow::answer1()));
    c.bench_function("d19_a2", |b| b.iter(|| go_with_the_flow::answer2()));

    c.bench_function("d20_a1", |b| b.iter(|| a_regular_map::answer1()));
    c.bench_function("d20_a2", |b| b.iter(|| a_regular_map::answer2()));

    c.bench_function("d21_a1", |b| b.iter(|| chronal_conversion::answer1()));
    c.bench_function("d21_a2", |b| b.iter(|| chronal_conversion::answer2()));

    c.bench_function("d22_a1", |b| b.iter(|| mode_maze::answer1()));
    c.bench_function("d22_a2", |b| b.iter(|| mode_maze::answer2()));

    c.bench_function("d23_a1", |b| {
        b.iter(|| experimental_emergency_teleportation::answer1())
    });

    c.bench_function("d24_a1", |b| b.iter(|| immune_system_simulator::answer1()));
    c.bench_function("d24_a2", |b| b.iter(|| immune_system_simulator::answer2()));

    c.bench_function("d25_a1", |b| {
        b.iter(|| four_dimensional_adventure::answer1())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
