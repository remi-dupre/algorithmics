const MAX: u32 = 999;

fn is_palindromic(mut x: u32) -> bool {
    let mut y = 0;

    while x >= y {
        if x == y {
            return true;
        }

        y *= 10;
        y += x % 10;
        x /= 10;
    }

    false
}

pub fn solve() -> u32 {
    (0..=MAX)
        .flat_map(|x| (0..=MAX).map(move |y| x * y))
        .filter(|prod| is_palindromic(*prod))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindromic() {
        assert!(is_palindromic(0));
        assert!(is_palindromic(11));
        assert!(is_palindromic(15344351));
    }
}
