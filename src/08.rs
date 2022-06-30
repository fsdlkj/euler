/// Largest product in a series

pub fn solve() -> i64 {
    // TODO: reuse inner
    include_str!("../data/08.txt")
        .replace('\n', "")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<_>>()
        .windows(13)
        .map(|xs| xs.iter().product())
        .max()
        .unwrap()
}
