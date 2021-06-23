use crate::util::arithmetic::Divisors;
use fxhash::FxHashSet;

const MAX: u64 = 28123;

fn is_abundant(n: u64) -> bool {
    n.divisors().filter(|k| *k < n).sum::<u64>() > n
}

pub fn solve() -> u64 {
    let abundants: Vec<_> = (1..=MAX).filter(|n| is_abundant(*n)).collect();

    let all_sums: FxHashSet<_> = abundants
        .iter()
        .flat_map(|x| abundants.iter().map(move |y| x + y))
        .filter(|n| *n <= MAX)
        .collect();

    let mut all_sums: Vec<_> = all_sums.into_iter().collect();
    all_sums.push(0);
    all_sums.push(MAX + 1);
    all_sums.sort_unstable();
    all_sums.windows(2).flat_map(|win| win[0] + 1..win[1]).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_abundant() {
        assert!(is_abundant(12));
        assert!(!is_abundant(28));
    }
}
