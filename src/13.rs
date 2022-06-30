/// Large sum

pub fn solve() -> i64 {
    let sum: f64 = include_str!("../data/13.txt")
        .lines()
        .map(|l| l[..10].parse::<f64>().unwrap())
        .sum();
    (sum / 10_0.).round() as i64
}
