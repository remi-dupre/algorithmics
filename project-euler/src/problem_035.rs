use crate::util::arithmetic::NbDigits;
use crate::util::primes::{is_prime, primes_bellow};
use fxhash::FxHashSet;

fn circle(n: u64) -> impl Iterator<Item = u64> {
    let nb_digits = n.nb_digits(10);

    (0..nb_digits).scan(n, move |n, _| {
        *n = (*n % 10) * 10u64.pow(u32::from(nb_digits) - 1) + (*n / 10);
        Some(*n)
    })
}

fn is_circular_prime(n: u64) -> bool {
    circle(n).all(is_prime)
}

pub fn solve() -> usize {
    let mut candidates: FxHashSet<_> = primes_bellow(1_000_000).collect();

    (2..=1_000_000)
        .filter(|n| {
            if candidates.contains(&n) {
                let res = is_circular_prime(*n);

                if !res {
                    for m in circle(*n) {
                        candidates.remove(&m);
                    }
                }

                res
            } else {
                false
            }
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_circular_prime() {
        assert!(is_circular_prime(7));
        assert!(is_circular_prime(79));
        assert!(is_circular_prime(197));
        assert!(!is_circular_prime(19));
    }
}
