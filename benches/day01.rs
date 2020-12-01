use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::days::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1-1", |b| b.iter(|| day01_1()));
    c.bench_function("day 1-2", |b| b.iter(|| day01_2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
