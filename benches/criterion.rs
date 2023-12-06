use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fibonacci::*;
use num_bigint::BigUint;
use num_traits::{One, Zero};

// Helper function to calculate Fibonacci the slow way for verification
fn slow_fibonacci(n: usize) -> BigUint {
    let (mut a, mut b) = (BigUint::zero(), BigUint::one());
    for _ in 0..n {
        (b, a) = (a + &b, b);
    }
    a
}

fn fibonacci_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    group
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_secs(1))
        .sample_size(32);
    for i in [0usize, 1, 10, 100, 1000, 10000, 25000, 50000, 75000, 100000].into_iter() {
        group.bench_with_input(BenchmarkId::new("Slow Naive", i), &i, |b, &i| {
            b.iter(|| slow_fibonacci(i))
        });
        group.bench_with_input(BenchmarkId::new("Fast Doubling", i), &i, |b, &i| {
            b.iter(|| fast_doubling_fibonacci(i))
        });
    }
    group.finish();
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
