use std::collections::HashSet;
use std::num::ParseIntError;

const TARGET: u32 = 2020;

pub fn generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

pub fn part_1_array(input: &[u32]) -> Option<u32> {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();

    for &num in &numbers {
        if numbers.binary_search(&(TARGET - num)).is_ok() {
            return Some(num * (TARGET - num));
        }
    }

    None
}

pub fn part_1_hashset(input: &[u32]) -> Option<u32> {
    let numbers: HashSet<_> = input.iter().copied().collect();

    for &num in input {
        if numbers.contains(&(TARGET - num)) {
            return Some(num * (TARGET - num));
        }
    }

    None
}

pub fn part_2(input: &[u32]) -> Option<u32> {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();

    for &x in &numbers {
        for &y in &numbers {
            if x + y <= TARGET && numbers.binary_search(&(TARGET - x - y)).is_ok() {
                return Some(x * y * (TARGET - x - y));
            }
        }
    }

    None
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day01::*;

    const EXAMPLE: &str = crate::lines! {
        "1721"
        "979"
        "366"
        "299"
        "675"
        "1456"
    };

    #[test]
    fn test_part_1() {
        let input = generator(EXAMPLE).unwrap();
        assert_eq!(Some(514579), part_1_array(&input));
        assert_eq!(Some(514579), part_1_hashset(&input));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(241861950), part_2(&generator(EXAMPLE).unwrap()));
    }
}
