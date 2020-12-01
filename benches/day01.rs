use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::days::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1-1 unsorted", |b| b.iter(|| day01_1(Sorting::Unsorted)));
    c.bench_function("day 1-2 unsorted", |b| b.iter(|| day01_2(Sorting::Unsorted)));
    c.bench_function("day 1-1 ascending", |b| b.iter(|| day01_1(Sorting::Ascending)));
    c.bench_function("day 1-2 ascending", |b| b.iter(|| day01_2(Sorting::Ascending)));
    c.bench_function("day 1-1 descending", |b| b.iter(|| day01_1(Sorting::Descending)));
    c.bench_function("day 1-2 descending", |b| b.iter(|| day01_2(Sorting::Descending)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
