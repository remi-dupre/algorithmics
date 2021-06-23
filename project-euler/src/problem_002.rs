use crate::util::sequences::fibonacci;

const MAX: u64 = 4_000_000;

pub fn solve() -> u64 {
    fibonacci::<u64>()
        .skip(1)
        .take_while(|x| *x < MAX)
        .filter(|x| x % 2 == 0)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let start: Vec<u64> = fibonacci().take(10).collect();
        assert_eq!(&start, &[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }
}
