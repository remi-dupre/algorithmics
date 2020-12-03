use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::iter;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<Section>, Vec<Section>) {
    let mut wires = input.lines().map(|line| {
        line.split(',')
            .map(|sec| sec.parse().expect("invalid section"))
            .collect()
    });

    (wires.next().unwrap(), wires.next().unwrap())
}

fn path(wire: &[Section]) -> impl Iterator<Item = (i64, i64)> + '_ {
    wire.iter()
        .flat_map(|section| iter::repeat(section.dir).take(section.len))
        .scan((0, 0), |pos, dir| {
            *pos = dir.apply(*pos);
            Some(*pos)
        })
}

#[aoc(day3, part1, hashset)]
pub fn part1_hashset(input: &(Vec<Section>, Vec<Section>)) -> u64 {
    let path1: HashSet<_> = path(&input.0).collect();
    let path2: HashSet<_> = path(&input.1).collect();

    path1
        .intersection(&path2)
        .map(|(x, y)| x.abs() as u64 + y.abs() as u64)
        .min()
        .expect("no intersection")
}

#[aoc(day3, part1, slice)]
pub fn part1_slice(input: &(Vec<Section>, Vec<Section>)) -> u64 {
    let mut path1: Vec<_> = path(&input.0).collect();
    let mut path2: Vec<_> = path(&input.1).collect();

    let abs = |(x, y): (i64, i64)| x.abs() as u64 + y.abs() as u64;
    let key = |pos| (abs(pos), pos);
    path1.sort_unstable_by(|&x, &y| key(x).cmp(&key(y)).reverse());
    path2.sort_unstable_by(|&x, &y| key(x).cmp(&key(y)).reverse());

    while path1.last() != path2.last() {
        let x = *path1.last().expect("no intersection");
        let y = *path2.last().expect("no intersection");

        if key(x) < key(y) {
            path1.pop();
        } else {
            path2.pop();
        }
    }

    abs(path1.pop().unwrap())
}

#[aoc(day3, part2)]
pub fn part2(input: &(Vec<Section>, Vec<Section>)) -> u64 {
    let path1: HashSet<_> = path(&input.0).collect();
    let path2: HashSet<_> = path(&input.1).collect();

    let steps1: HashMap<_, _> = path(&input.0)
        .enumerate()
        .map(|(steps, pos)| (pos, 1 + steps as u64))
        .collect();

    let steps2: HashMap<_, _> = path(&input.1)
        .enumerate()
        .map(|(steps, pos)| (pos, 1 + steps as u64))
        .collect();

    path1
        .intersection(&path2)
        .map(|pos| steps1[pos] + steps2[pos])
        .min()
        .expect("no intersection")
}

// ---
// --- Structs
// ---

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Section {
    dir: Direction,
    len: usize,
}

impl Direction {
    fn apply(&self, pos: (i64, i64)) -> (i64, i64) {
        let (x, y) = pos;

        match self {
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        }
    }
}

// ---
// --- Structs Deserialization
// ---

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "R" => Self::Right,
            "L" => Self::Left,
            _ => return Err("invalid direction"),
        })
    }
}

impl FromStr for Section {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            dir: s[0..1].parse()?,
            len: s[1..].parse()?,
        })
    }
}
