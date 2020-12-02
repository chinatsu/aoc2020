use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::days::*;
use aoc2020::lines_from;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut entries = lines_from!("01", i32);
    c.bench_function("parser day 1", |b| b.iter(|| lines_from!("01", i32)));
    c.bench_function("day 1-1", |b| b.iter(|| day01_1(&mut entries)));
    c.bench_function("day 1-2", |b| b.iter(|| day01_2(&mut entries)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
