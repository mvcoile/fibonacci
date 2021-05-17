/*
In mathematics, the Fibonacci numbers, commonly denoted Fn, form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1.
- F_{0}=0, F_{1}=1
- F_{n}=F_{n-1}+F_{n-2}

https://www.nayuki.io/page/fast-fibonacci-algorithms

Fast doubling:
*/
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

fn main() {
    let f: usize = 4096;

    let now = std::time::Instant::now();
    println!("fib {}: {} ({} microseconds)", f, fibonacci(f), now.elapsed().as_micros());
}

fn fibonacci(n: usize) -> BigUint {
    _fib(n.to_biguint().unwrap()).0
}

fn _fib(n: BigUint) -> (BigUint, BigUint) {
    if n.is_zero() {
        return (Zero::zero(), One::one());
    } else {
        let (a, b) = _fib(&n / 2.to_biguint().unwrap()); // ignoring remainder. eg. 17/7 = 2
        let c: BigUint = &a * (&b * 2.to_biguint().unwrap() - &a);
        let d: BigUint = a.pow(2) + b.pow(2);

        if n % 2.to_biguint().unwrap() == Zero::zero() {
            return (c, d);
        } else {
            return (d.clone(), c + d);
        }
    }
}
