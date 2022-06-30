/// Maximum path sum I

pub fn solve() -> i64 {
    // lol
    *include_str!("../data/18.txt")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .fold([0; 16], |mut acc, num| {
            for (i, n) in num.iter().enumerate() {
                acc[i] = n + std::cmp::max(acc[i], acc[i + 1]);
            }
            acc
        })
        .first()
        .unwrap() as i64
}
