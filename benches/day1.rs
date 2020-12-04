use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::days::day1::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut entries = parser::parse(INPUT);
    c.bench_function("parser day 1", |b| b.iter(|| parser::parse(INPUT)));
    c.bench_function("day 1-1", |b| b.iter(|| one(&mut entries)));
    c.bench_function("day 1-2", |b| b.iter(|| two(&mut entries)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
