use crate::util::arithmetic::binomial;

const SIZE: u64 = 20;

pub fn solve() -> u64 {
    binomial(SIZE, SIZE * 2)
}
