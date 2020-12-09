use criterion::{criterion_group, criterion_main, Criterion};
use aoc2020::days::day9::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let entries = parser::parse(INPUT);
    let target = solve_one(&entries, 25);
    c.bench_function("parser day 9", |b| b.iter(|| parser::parse(INPUT)));
    c.bench_function("day 9-1", |b| b.iter(|| solve_one(&entries, 25)));
    c.bench_function("day 9-2", |b| b.iter(|| solve_two(&entries, target)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
