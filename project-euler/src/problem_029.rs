use fxhash::FxHashSet;
use num_bigint::ToBigUint;

pub fn solve() -> usize {
    ((2..=100).flat_map(|a| (2..=100).map(move |b| (a, b))))
        .map(|(a, b)| a.to_biguint().unwrap().pow(b))
        .collect::<FxHashSet<_>>()
        .len()
}
