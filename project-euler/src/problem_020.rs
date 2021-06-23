use crate::util::arithmetic::Digits;
use num_bigint::{BigUint, ToBigUint};

pub fn solve() -> u64 {
    let factorial: BigUint = (1..=100).map(|x| x.to_biguint().unwrap()).product();
    factorial.digits(10).map(u64::from).sum()
}
