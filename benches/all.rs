#[macro_use]
extern crate criterion;

use aoc::day01::chronal_calibration;
use aoc::day02::inventory_management_system;
use aoc::day03::no_matter_how_you_slice_it;
use aoc::day04::repose_record;
use aoc::day05::alchemical_reduction;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1 answer 1", |b| {
        b.iter(|| chronal_calibration::answer1())
    });
    c.bench_function("day 1 answer 2", |b| {
        b.iter(|| chronal_calibration::answer2())
    });

    c.bench_function("day 2 answer 1", |b| {
        b.iter(|| inventory_management_system::answer1())
    });
    c.bench_function("day 2 answer 2", |b| {
        b.iter(|| inventory_management_system::answer2())
    });

    c.bench_function("day 3 answer 1", |b| {
        b.iter(|| no_matter_how_you_slice_it::answer1())
    });
    c.bench_function("day 3 answer 2", |b| {
        b.iter(|| no_matter_how_you_slice_it::answer2())
    });

    c.bench_function("day 4 answer 1", |b| b.iter(|| repose_record::answer1()));
    c.bench_function("day 4 answer 2", |b| b.iter(|| repose_record::answer2()));

    c.bench_function("day 5 answer 1", |b| {
        b.iter(|| alchemical_reduction::answer1())
    });
    c.bench_function("day 5 answer 2", |b| {
        b.iter(|| alchemical_reduction::answer2())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
