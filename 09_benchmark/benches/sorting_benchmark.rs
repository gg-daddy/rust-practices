use benchmark::{sort_algo_1, sort_algo_2};

use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers: Vec<i32> = vec![
        1, 2, 3, 6, 5, 4, 8, 52, 2, 1, 5, 4, 4, 5, 8, 54, 2, 0, 55, 5, 2, 0, 5, 5, 5, 21,
    ];

    // This creates a benchmark
    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_1(&mut numbers))
    });    // This creates a benchmark
    c.bench_function("Sorting Algorithm 2", |b| {
        b.iter(|| sort_algo_2(&mut numbers))
    });
}

fn sort_benchmark2(c: &mut Criterion) {
    let mut numbers: Vec<i32> = vec![
        1, 2, 3, 6, 5, 4, 8, 52, 2, 1, 5, 4, 4, 5, 8, 54, 2, 0, 55, 5, 2, 0, 5, 5, 5, 21,
    ];

    // This creates a benchmark
    c.bench_function("Sorting Algorithm 3", |b| {
        b.iter(|| sort_algo_1(&mut numbers))
    });    // This creates a benchmark
    c.bench_function("Sorting Algorithm 4", |b| {
        b.iter(|| sort_algo_2(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_group!(benches1, sort_benchmark2);
criterion_main!(benches, benches1);