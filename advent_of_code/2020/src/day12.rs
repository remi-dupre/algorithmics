use std::convert::{TryFrom, TryInto};
use std::error::Error;

use crate::utils::Point2D;

pub fn generator(input: &str) -> Result<Vec<Op>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            if line.len() < 2 {
                return Err("empty line".into());
            }
            let (cmd, val) = line.split_at(1);

            match cmd {
                "E" | "S" | "W" | "N" => Ok(Op::Direction(cmd.try_into()?, val.parse()?)),
                "F" => Ok(Op::Forward(val.parse()?)),
                "R" | "L" => {
                    let rot: i16 = val.parse()?;

                    if rot % 90 != 0 {
                        return Err(format!("").into());
                    }

                    if cmd == "R" {
                        Ok(Op::Rotate((rot / 90).try_into()?))
                    } else {
                        Ok(Op::Rotate((4 - rot / 90).try_into()?))
                    }
                }
                other => Err(format!("invalid operation `{}`", other).into()),
            }
        })
        .collect()
}

pub fn part_1(operations: &[Op]) -> i32 {
    let (_, pos) = operations.iter().fold(
        (Card::East, Point2D::new(0, 0)),
        |(curr_dir, pos), op| match *op {
            Op::Forward(dist) => (curr_dir, pos + curr_dir.as_offset().mul(dist)),
            Op::Direction(dir, dist) => (curr_dir, pos + dir.as_offset().mul(dist)),
            Op::Rotate(rot) => ((0..rot).fold(curr_dir, |dir, _| dir.next()), pos),
        },
    );

    pos.x.abs() + pos.y.abs()
}

pub fn part_2(operations: &[Op]) -> i32 {
    let (ship, _) = operations.iter().fold(
        (Point2D::new(0, 0), Point2D::new(10, 1)),
        |(ship, wp), op| match *op {
            Op::Forward(dist) => (ship + wp.mul(dist), wp),
            Op::Direction(dir, dist) => (ship, wp + dir.as_offset().mul(dist)),
            Op::Rotate(rot) => (ship, (0..rot).fold(wp, |wp, _| Point2D::new(wp.y, -wp.x))),
        },
    );

    ship.x.abs() + ship.y.abs()
}

// ---
// --- Structs
// ---

pub enum Op {
    Forward(i32),
    Rotate(u8),
    Direction(Card, i32),
}

#[derive(Clone, Copy)]
pub enum Card {
    East,
    South,
    West,
    North,
}

impl Card {
    fn as_offset(self) -> Point2D<i32> {
        match self {
            Self::East => Point2D::new(1, 0),
            Self::South => Point2D::new(0, -1),
            Self::West => Point2D::new(-1, 0),
            Self::North => Point2D::new(0, 1),
        }
    }

    fn next(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "N" => Ok(Self::North),
            "S" => Ok(Self::South),
            "E" => Ok(Self::East),
            "W" => Ok(Self::West),
            _ => Err(format!("unknown cardinal `{}`", value)),
        }
    }
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day12::*;

    const EXAMPLE: &str = crate::lines! {
        "F10"
        "N3"
        "F7"
        "R90"
        "F11"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(25, part_1(&generator(EXAMPLE).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(286, part_2(&generator(EXAMPLE).unwrap()));
    }
}
