use crate::util::arithmetic::Digits;

pub fn solve() -> u64 {
    (2..1_000_000)
        .filter(|&n| n == n.digits_rev(10).map(|d| u64::from(d).pow(5)).sum())
        .sum()
}
