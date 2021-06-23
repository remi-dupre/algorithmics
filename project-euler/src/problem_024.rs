use crate::util::algorithms::Permutations;

pub fn solve() -> u64 {
    9876543210.permutations().nth(1_000_001).unwrap()
}
