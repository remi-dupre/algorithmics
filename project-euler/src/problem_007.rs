use crate::util::primes::primes;

const INDEX: usize = 10_001;

pub fn solve() -> u64 {
    primes().nth(INDEX - 1).unwrap()
}
