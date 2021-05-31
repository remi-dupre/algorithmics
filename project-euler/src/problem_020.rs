use num_bigint::{BigUint, ToBigUint};

pub fn solve() -> u64 {
    let factorial: BigUint = (1..=100).map(|x| x.to_biguint().unwrap()).product();
    factorial.to_radix_le(10).into_iter().map(u64::from).sum()
}
