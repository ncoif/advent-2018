#[macro_use]
extern crate criterion;

use aoc::day01::chronal_calibration::{answer1, answer2};

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("answer 1", |b| b.iter(|| answer1()));
    c.bench_function("answer 2", |b| b.iter(|| answer2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
