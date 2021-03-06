use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day01_benchmark(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::solve()));
}

pub fn day02_benchmark(c: &mut Criterion) {
    c.bench_function("day02", |b| b.iter(|| day02::solve()));
}

pub fn day03_benchmark(c: &mut Criterion) {
    c.bench_function("day03", |b| b.iter(|| day03::solve()));
}

pub fn alldays_benchmark(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| (day01::solve(), day02::solve(), day03::solve()))
    });
}

criterion_group! {
    name = benches;

    config = Criterion::default()
        .significance_level(0.1)
        .sample_size(500)
        .measurement_time(Duration::from_secs(20))
        .warm_up_time(Duration::from_secs(5))
        .noise_threshold(0.05);

    targets =
        day01_benchmark,
        day02_benchmark,
        day03_benchmark,
        alldays_benchmark
}

criterion_main!(benches);
