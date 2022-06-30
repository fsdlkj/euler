/// Smallest multiple

fn gcd(mut a: i64, mut b: i64) -> i64 {
    // Euclid's algorithm
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }

    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

pub fn solve() -> i64 {
    let mut res = 1;

    for i in 2..20 {
        res = lcm(res, i);
    }

    res
}
