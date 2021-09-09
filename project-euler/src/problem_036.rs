use crate::util::arithmetic::Digits;
use rayon::prelude::*;

pub fn solve() -> u64 {
    (1..1_000_000)
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter(|x| x.digits(10).zip(x.digits_rev(10)).all(|(x, y)| x == y))
        .filter(|x| x.digits(2).zip(x.digits_rev(2)).all(|(x, y)| x == y))
        .sum()
}
