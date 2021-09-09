use crate::util::primes;

pub fn solve() -> u64 {
    primes::is_prime(100_000_000);
    // primes::primes().for_each(|p| println!("{}", p));
    0
}
