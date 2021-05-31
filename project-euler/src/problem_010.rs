use crate::util::primes::primes_bellow;

const MAX: u64 = 2_000_000;

pub fn solve() -> u64 {
    primes_bellow(MAX).sum()
}
