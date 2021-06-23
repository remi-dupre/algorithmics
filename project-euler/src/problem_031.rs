use cached::proc_macro::cached;
use std::cmp::min;

const COINS: [u8; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

#[cached]
fn count_combinations(target: u8, max: u8) -> u64 {
    if target == 0 {
        1
    } else {
        COINS
            .iter()
            .copied()
            .take_while(|c| *c <= min(target, max))
            .map(|c| count_combinations(target - c, c))
            .sum()
    }
}

pub fn solve() -> u64 {
    count_combinations(200, 200)
}
