use fibonacci::*;
use rayon::prelude::*;
use std::iter::repeat_with;

fn main() {
    let fib_indexes: Vec<usize> = repeat_with(|| fastrand::usize(1_000_000..10_000_000))
        .take(10)
        .collect();
    println!("fib_indexes: {fib_indexes:?}");

    let now = std::time::Instant::now();
    fib_indexes.clone().into_par_iter().for_each(|i| {
        let _fib = recursive_fibonacci(i);
    });
    let dur = now.elapsed().as_secs_f64().to_string();
    println!("fibonacci: ({dur} seconds)");

    let now = std::time::Instant::now();
    fib_indexes.clone().into_par_iter().for_each(|i| {
        let _fib = fast_doubling_fibonacci(i);
        //println!("fib {f}: {_fib} ({dur} microseconds)");
    });
    let dur = now.elapsed().as_secs_f64().to_string();
    println!("fast_doubling_fibonacci: ({dur} seconds)");
}
