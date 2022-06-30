/// Special Pythagorean triplet

pub fn solve() -> i64 {
    let upper_m = (1000 as f64).sqrt() as i64;
    let upper_n = ((1000 as f64).sqrt() / 2.) as i64;

    for m in 0..upper_m {
        for n in 0..upper_n {
            let a = m.pow(2) - n.pow(2);
            let b = 2 * m * n;
            let c = m.pow(2) + n.pow(2);
            if a + b + c == 1000 {
                return a * b * c;
            }
        }
    }

    unsafe {
        std::hint::unreachable_unchecked();
    }
}
