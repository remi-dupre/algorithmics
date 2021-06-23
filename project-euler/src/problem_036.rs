use crate::util::arithmetic::Digits;

pub fn solve() -> u64 {
    (1..1_000_000)
        .filter(|x| x.digits(10).zip(x.digits_rev(10)).all(|(x, y)| x == y))
        .filter(|x| x.digits(2).zip(x.digits_rev(2)).all(|(x, y)| x == y))
        .sum()
}
