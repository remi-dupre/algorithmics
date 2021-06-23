use num_integer::binomial;

const SIZE: u64 = 20;

pub fn solve() -> u64 {
    binomial(SIZE * 2, SIZE)
}
