const MAX: u64 = 20;

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn solve() -> u64 {
    (1..=MAX).fold(1, |x, y| x * y / gcd(x, y))
}
