use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::days::day02::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut entries = passwords_from("02");
    c.bench_function("parser day 2", |b| b.iter(|| passwords_from("02")));
    c.bench_function("day 2-1", |b| b.iter(|| solve_day02_1(&mut entries)));
    c.bench_function("day 2-2", |b| b.iter(|| solve_day02_2(&mut entries)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
