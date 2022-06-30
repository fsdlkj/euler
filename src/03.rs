/// Largest prime factor

pub fn solve() -> i64 {
    let mut n = 600851475143i64;
    let mut i = 2;

    while n > 1 {
        if n % i == 0 {
             n /= i;
        } else {
            i += 1
        }
    }

    return i as i64
}
