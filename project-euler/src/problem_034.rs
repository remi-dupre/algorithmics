use crate::util::arithmetic::Digits;

pub fn solve() -> u64 {
    let factorial = |n| -> u64 { (1..=n).product() };

    (10..100_000)
        .filter(|&n| n == n.digits(10).map(u64::from).map(factorial).sum())
        .sum()
}
