use crate::util::primes::primes_bellow;

const INPUT: u64 = 600_851_475_143;

pub fn solve() -> u64 {
    let sqrt = (INPUT as f64).sqrt() as u64;

    primes_bellow(sqrt)
        .filter(|p| INPUT % p == 0)
        .last()
        .unwrap()
}
