use crate::util::primes::is_prime;
use std::convert::TryFrom;

fn consecutive_primes(a: i64, b: i64) -> i64 {
    (0..)
        .find(|&n| {
            u64::try_from((n * (n + a)) + b)
                .map(|u| !is_prime(u))
                .unwrap_or(true)
        })
        .unwrap()
}

pub fn solve() -> i64 {
    ((-999..=999).flat_map(|a| (-1000..=1000).map(move |b| (a, b))))
        .max_by_key(|(a, b)| consecutive_primes(*a, *b))
        .map(|(a, b)| a * b)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_consecutive_primes() {
        assert_eq!(consecutive_primes(-79, 1601), 80);
    }
}
