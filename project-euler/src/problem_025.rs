use crate::util::sequences::fibonacci;
use num_bigint::BigUint;

pub fn solve() -> usize {
    1 + fibonacci::<BigUint>()
        .enumerate()
        .find(|(_, val)| val.to_radix_le(10).len() >= 1_000)
        .unwrap()
        .0
}
