const MAX: u64 = 4_000_000;

fn fibonacci() -> impl Iterator<Item = u64> {
    let (mut u, mut v) = (1, 1);

    std::iter::from_fn(move || {
        let w = u + v;
        u = v;
        v = w;
        Some(u)
    })
}

pub fn solve() -> u64 {
    fibonacci()
        .take_while(|x| *x < MAX)
        .filter(|x| x % 2 == 0)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let start: Vec<_> = fibonacci().take(10).collect();
        assert_eq!(&start, &[1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }
}
