/// Largest exponential

pub fn solve() -> i64 {
    let (mut lineno, mut highline, mut highest) = (0, 0, 0.);

    include_str!("../data/99.txt")
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .for_each(|(b, e)| {
            let val = e.parse::<f64>().unwrap() * b.parse::<f64>().unwrap().log(10.0);
            lineno += 1;
            if val > highest {
                (highest, highline) = (val, lineno);
            }
        });

    highline
}
