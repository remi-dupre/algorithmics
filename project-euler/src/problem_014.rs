use fxhash::FxBuildHasher;
use std::collections::HashMap;

const MAX: u64 = 1_000_000;

fn collatz_iter(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn collatz_chain_size(cache: &mut HashMap<u64, usize, FxBuildHasher>, init: u64) -> usize {
    let parent = collatz_iter(init);

    if let Some(length) = cache.get(&parent) {
        length.wrapping_add(1)
    } else {
        cache.insert(init, usize::MAX);
        let length = collatz_chain_size(cache, parent);
        cache.insert(init, length.wrapping_add(1));
        length.wrapping_add(1)
    }
}

pub fn solve() -> u64 {
    let mut cache = HashMap::with_hasher(FxBuildHasher::default());

    (1..MAX)
        .max_by_key(|init| collatz_chain_size(&mut cache, *init))
        .unwrap()
}
