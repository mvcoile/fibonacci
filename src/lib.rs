use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

/*
In mathematics, the Fibonacci numbers, commonly denoted Fn, form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1.
 - F(0) = 0, F(1) = 1
 - F(n) = F(n-1) + F(n-2)

* Fast doubling method. Faster than the naive method.
 - F(2n)   = F(n) * (2*F(n+1) - F(n)).
 - F(2n+1) = F(n+1)^2 + F(n)^2.

 This method relies on the correlation between the F(n) and F(2n). Allowing us to skip all calculations between F(n) and F(2n). Making this method O(log2 n).

https://www.nayuki.io/page/fast-fibonacci-algorithms
*/

pub fn recursive_fibonacci(n: usize) -> BigUint {
    _fib(n.to_biguint().unwrap()).0
}

fn _fib(n: BigUint) -> (BigUint, BigUint) {
    if n.is_zero() {
        (Zero::zero(), One::one())
    } else {
        let (a, b) = _fib(&n / 2.to_biguint().unwrap()); // ignoring remainder. eg. 17/7 = 2
        let c: BigUint = &a * (&b * 2.to_biguint().unwrap() - &a);
        let d: BigUint = a.pow(2) + b.pow(2);

        if n % 2.to_biguint().unwrap() == Zero::zero() {
            (c, d)
        } else {
            (d.clone(), c + d)
        }
    }
}

pub fn fast_doubling_fibonacci(index: usize) -> BigUint {
    let (mut a, mut b) = (BigUint::zero(), BigUint::one());

    // Calculate the first 1 bit in index.
    let mut bit = 2usize.pow(index.ilog2());
    // Walking down the bits of index. When the bit is 0, we only need to calculate a and b using the doubling method. When the bit is 1, we do an extra naive step. This is because the index is uneven
    while bit != 0 {
        (a, b) = (&a * ((&b << 1) - &a), a.pow(2) + b.pow(2));

        if (index & bit) != 0 {
            (a, b) = (b.clone(), a + b);
        }

        bit >>= 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to calculate Fibonacci the slow way for verification
    fn slow_fibonacci(n: usize) -> BigUint {
        let mut a: BigUint = BigUint::one();
        let mut b: BigUint = BigUint::one();
        for _ in 0..n {
            (a, b) = (b.clone(), a + b);
        }
        a
    }

    #[test]
    fn test_fast_doubling_fibonacci_1() {
        let result = fast_doubling_fibonacci(522);
        let expected = slow_fibonacci(522);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_2() {
        let result = fast_doubling_fibonacci(1450);
        let expected = slow_fibonacci(1450);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_3() {
        let result = fast_doubling_fibonacci(2333);
        let expected = slow_fibonacci(2333);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_4() {
        let result = fast_doubling_fibonacci(987);
        let expected = slow_fibonacci(987);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_5() {
        let result = fast_doubling_fibonacci(3210);
        let expected = slow_fibonacci(3210);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_6() {
        let result = fast_doubling_fibonacci(4044);
        let expected = slow_fibonacci(4044);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_7() {
        let result = fast_doubling_fibonacci(2741);
        let expected = slow_fibonacci(2741);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_8() {
        let result = fast_doubling_fibonacci(1690);
        let expected = slow_fibonacci(1690);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_9() {
        let result = fast_doubling_fibonacci(3587);
        let expected = slow_fibonacci(3587);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fast_doubling_fibonacci_10() {
        let result = fast_doubling_fibonacci(4912);
        let expected = slow_fibonacci(4912);
        assert_eq!(result, expected);
    }
}
