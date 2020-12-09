use std::error::Error;
use std::str::FromStr;

pub fn generator(input: &str) -> Result<Vec<(Policy, &str)>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, ": ");
            Ok((
                parts.next().ok_or("missing policy")?.parse()?,
                parts.next().ok_or("missing password")?,
            ))
        })
        .collect()
}

pub fn part_1(input: &[(Policy, &str)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            let count = bytecount::count(password.as_bytes(), policy.letter);
            (policy.start..=policy.end).contains(&count)
        })
        .count()
}

pub fn part_2(input: &[(Policy, &str)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            let bytes = password.as_bytes();
            (bytes[policy.start - 1] == policy.letter) ^ (bytes[policy.end - 1] == policy.letter)
        })
        .count()
}

// ---
// --- Structs
// ---

pub struct Policy {
    letter: u8,
    start: usize,
    end: usize,
}

impl FromStr for Policy {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<_>>().as_slice() {
            [range, letter] => {
                let (start, end) = match range.split('-').collect::<Vec<_>>().as_slice() {
                    [start, end] => (start.parse()?, end.parse()?),
                    _ => return Err("range should contain exactly one `-`".into()),
                };

                let letter = match letter.as_bytes() {
                    [letter] => *letter,
                    _ => return Err("only single ascii letters are allowed in policy".into()),
                };

                Ok(Self { letter, start, end })
            }
            _ => Err("policy should contain exactly one space".into()),
        }
    }
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day2::*;

    const EXAMPLE: &str = crate::lines! {
        "1-3 a: abcde"
        "1-3 b: cdefg"
        "2-9 c: ccccccccc"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1(&generator(EXAMPLE).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1, part_2(&generator(EXAMPLE).unwrap()));
    }
}
