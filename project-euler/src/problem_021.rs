use crate::util::arithmetic::divisors;

fn is_amicable(x: u64) -> bool {
    let d = |n| divisors(n).filter(|d| *d < n).sum();
    x > 1 && d(x) != x && d(d(x)) == x
}

pub fn solve() -> u64 {
    (1..10_000).filter(|x| is_amicable(*x)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_amicable() {
        assert!(is_amicable(220));
        assert!(is_amicable(284));
        assert!(is_amicable(1184));
        assert!(is_amicable(5020));
        assert!(is_amicable(6232));
        assert!(!is_amicable(42));
        assert!(!is_amicable(4000));
    }
}
