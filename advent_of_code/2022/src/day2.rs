use std::cmp::Ordering;

use anyhow::{anyhow, bail, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn strength(self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn weakness(self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn score(self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some({
            if other == self {
                Ordering::Equal
            } else if other == &self.strength() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
    }
}

#[derive(Clone, Copy)]
pub enum Tip {
    X,
    Y,
    Z,
}

impl Tip {
    fn into_move(self) -> Move {
        match self {
            Tip::X => Move::Rock,
            Tip::Y => Move::Paper,
            Tip::Z => Move::Scissors,
        }
    }

    fn into_move_for_outcome(self, opponent: Move) -> Move {
        match self {
            Tip::X => opponent.strength(),
            Tip::Y => opponent,
            Tip::Z => opponent.weakness(),
        }
    }
}

pub fn generator(input: &str) -> Result<Vec<(Tip, Move)>> {
    input
        .lines()
        .map(|line| {
            let (other, yours) = line
                .trim_end()
                .split_once(' ')
                .ok_or_else(|| anyhow!("missing space separator"))?;

            let other = match other {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                x => bail!("invalid move '{x}"),
            };

            let yours = match yours {
                "X" => Tip::X,
                "Y" => Tip::Y,
                "Z" => Tip::Z,
                x => bail!("invalid move '{x}"),
            };

            Ok((yours, other))
        })
        .collect()
}

pub fn points(yours: Move, other: Move) -> u64 {
    yours.score()
        + match yours.cmp(&other) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
}

pub fn part1(rounds: &[(Tip, Move)]) -> u64 {
    rounds
        .iter()
        .map(|&(tip, other)| points(tip.into_move(), other))
        .sum()
}

pub fn part2(rounds: &[(Tip, Move)]) -> u64 {
    rounds
        .iter()
        .map(|&(tip, other)| points(tip.into_move_for_outcome(other), other))
        .sum()
}
