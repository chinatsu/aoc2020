use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::days::*;
use aoc2020::lines_from;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut entries = lines_from!("01", i32);
    c.bench_function("day 1-1 unsorted", |b| b.iter(|| day01_1(&mut entries, Sorting::Unsorted)));
    c.bench_function("day 1-2 unsorted", |b| b.iter(|| day01_2(&mut entries, Sorting::Unsorted)));
    c.bench_function("day 1-1 ascending", |b| b.iter(|| day01_1(&mut entries, Sorting::Ascending)));
    entries = lines_from!("01", i32); // refreshing the contents to keep the sorting stuff realistic
    c.bench_function("day 1-2 ascending", |b| b.iter(|| day01_2(&mut entries, Sorting::Ascending)));
    entries = lines_from!("01", i32);
    c.bench_function("day 1-1 descending", |b| b.iter(|| day01_1(&mut entries, Sorting::Descending)));
    entries = lines_from!("01", i32);
    c.bench_function("day 1-2 descending", |b| b.iter(|| day01_2(&mut entries, Sorting::Descending)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
