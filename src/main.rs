/*
In mathematics, the Fibonacci numbers, commonly denoted Fn, form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1.
- F_{0}=0, F_{1}=1
- F_{n}=F_{n-1}+F_{n-2}

https://www.nayuki.io/page/fast-fibonacci-algorithms

Fast doubling:
*/
fn main() {
    let f: u128 = 128;

    let now = std::time::Instant::now();
    println!("fib {}: {} ({} ns)", f, fibonacci(f), now.elapsed().as_nanos());

}

fn fibonacci(n: u128) -> u128 {
    _fib(n).0
}

fn _fib(n: u128) -> (u128, u128) {
    if n == 0 {
        (0,1)
    } else {
        let (a, b) = _fib(n / 2); // ignoring remainder. eg.17/7 = 2
        let c: u128 = a * (b * 2 - a);
        let d: u128 = a * a + b * b;
        if n % 2 == 0 {
            (c, d)
        } else {
            (d, c+d)
        }
    }
}
