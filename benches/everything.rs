use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("everything", |b| b.iter(|| aoc2020::main()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
