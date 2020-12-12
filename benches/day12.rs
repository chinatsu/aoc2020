use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::days::day12::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let entries = parser::parse(INPUT);
    c.bench_function("parser day 12", |b| b.iter(|| parser::parse(INPUT)));
    c.bench_function("day 12-1", |b| b.iter(|| one(&entries)));
    c.bench_function("day 12-2", |b| b.iter(|| two(&entries)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
