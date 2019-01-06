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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
