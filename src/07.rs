/// 10001st prime

pub fn nth_prime(n: i64) -> i64 {
    // p_n < n lg(n + ln(lg(n))) for n >= 6
    let m = n as f32;
    let upper_bound = (m * (m.ln() + (m.ln()).ln())) as usize;

    let mut sieve = vec![true; upper_bound];
    // handle the special case p = 2
    let mut num_primes_found = 1;

    // so that we only consider odd numbers
    for p in (3..).step_by(2) {
        if unsafe { !sieve.get_unchecked(p) } {
            continue;
        } else {
            num_primes_found += 1;
            if num_primes_found == n {
                return p as i64;
            }
            for pp in ((p * p)..upper_bound).step_by(p) {
                unsafe { *sieve.get_unchecked_mut(pp) = false };
            }
        }
    }
    unsafe { std::hint::unreachable_unchecked() }
}

pub fn solve() -> i64 {
    nth_prime(10001)
}
