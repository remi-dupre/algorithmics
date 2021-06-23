use cached::proc_macro::cached;
use std::convert::TryInto;

#[cached]
fn sieve(max: u64) -> Vec<bool> {
    assert!(max.is_power_of_two());

    let max: usize = max.try_into().unwrap();
    let mut prime = vec![true; max + 1];
    prime[0] = false;
    prime[1] = false;

    for x in 2..=max {
        if !prime[x] {
            continue;
        }

        prime[x..]
            .iter_mut()
            .step_by(x)
            .skip(1)
            .for_each(|v| *v = false);
    }

    prime
}

pub fn primes_bellow(max: u64) -> impl Iterator<Item = u64> {
    sieve(max.next_power_of_two())
        .into_iter()
        .enumerate()
        .filter(|(_, prime)| *prime)
        .map(|(x, _)| x.try_into().unwrap())
}

pub fn primes() -> impl Iterator<Item = u64> {
    let mut prev_max = 0;
    let mut max = 8;

    std::iter::from_fn(move || {
        let res = primes_bellow(max)
            .into_iter()
            .skip_while(move |p| *p <= prev_max);

        // Double the size of the sieve for next iteration
        prev_max = max;
        max *= 2;

        Some(res)
    })
    .flatten()
}

pub fn is_prime(n: u64) -> bool {
    sieve(n.next_power_of_two())[n as usize]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_primes_bellow() {
        let start: Vec<_> = primes_bellow(13).collect();
        assert_eq!(&start, &[2, 3, 5, 7, 11, 13]);
    }

    #[test]
    fn test_primes() {
        let start: Vec<_> = primes().take(6).collect();
        assert_eq!(&start, &[2, 3, 5, 7, 11, 13]);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(13));
        assert!(!is_prime(42));
    }
}
