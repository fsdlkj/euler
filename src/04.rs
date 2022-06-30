/// Largest palindrome product

fn palindrome_p(mut n: i64) -> bool {
    let original = n;
    let mut rev = 0;

    while n > 0 {
        rev = rev * 10 + n % 10;
        n = n / 10;
    }

    rev == original
}

pub fn solve() -> i64 {
    let mut cur_max = 0;

    for p in (99..=999).rev() {
        for q in (99..=999).rev() {
            let n = p * q;
            if n < cur_max {
                break;
            } else if palindrome_p(n) {
                cur_max = n;
                break;
            }
        }
    }

    cur_max as i64
}
