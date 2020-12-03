use std::error::Error;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Policy, String)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, ": ");
            (
                parts
                    .next()
                    .expect("missing policy")
                    .parse()
                    .unwrap_or_else(|err| panic!("invalid policy for `{}`: `{}`", line, err)),
                parts.next().expect("missing password").to_string(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            let count = bytecount::count(password.as_bytes(), policy.letter);
            (policy.start..=policy.end).contains(&count)
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Policy, String)]) -> usize {
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
