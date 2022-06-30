/// Summation of primes
use crate::utils::primes;

pub fn solve() -> i64 {
    let mut sum = 0;
    let mut iter = primes(2_000_000);
    while let Some(p) = iter.next() {
        sum += p;
    }
    sum as i64
}
