/// Lattice paths

// TIL
// https://stemhash.com/counting-lattice-paths/

// Taken from rosettacode
fn fact(n: u32) -> u128 {
    let mut f: u128 = n as u128;
    for i in 2..n {
        f *= i as u128;
    }
    return f;
}

// Ditto
fn choose(n: u32, k: u32) -> u128 {
    let mut num: u128 = n as u128;
    for i in 1..k {
        num *= (n - i) as u128;
    }
    return num / fact(k);
}

pub fn solve() -> i64 {
    choose(40, 20) as i64
}
