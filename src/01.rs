/// Multiples of 3 or 5

fn sum_multiples(k: i64, upto: i64) -> i64 {
    let n = upto / k;
    k * n * (n + 1) / 2
}

pub fn solve() -> i64 {
    sum_multiples(3, 999) + sum_multiples(5, 999) - sum_multiples(15, 999)
}
