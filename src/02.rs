/// Even Fibonacci numbers

pub fn solve() -> i64 {
    let limit = 4000000;
    // The first two even Fibonacci numbers (2, 8)
    let (mut a, mut b) = (2, 8);
    let mut sum = 10;

    // The insight is that F_n = 4 * F_(n-1) + F_(n-2)
    while a + 4 * b < limit {
        (a, b) = (b, a + 4 * b);
        sum += b;
    }

    sum
}
