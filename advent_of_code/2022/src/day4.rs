use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};

pub fn generator(input: &str) -> Result<Vec<[RangeInclusive<u32>; 2]>> {
    input
        .lines()
        .map(|line| {
            let assignments: Result<Vec<RangeInclusive<u32>>> = (line.trim_end())
                .split(',')
                .map(|part| {
                    let (x, y) = part
                        .split_once('-')
                        .ok_or_else(|| anyhow!("missing dash in assignment"))?;

                    Ok(x.parse()?..=y.parse()?)
                })
                .collect();

            assignments?
                .try_into()
                .map_err(|_| anyhow!("should contain exactly one assignment per line:"))
        })
        .collect()
}

fn contains(outer: &RangeInclusive<u32>, inner: &RangeInclusive<u32>) -> bool {
    outer.start() <= inner.start() && outer.end() >= inner.end()
}

fn overlap(x: &RangeInclusive<u32>, y: &RangeInclusive<u32>) -> bool {
    y.start() <= x.end() && x.start() <= y.end()
}

pub fn part1(assignments: &[[RangeInclusive<u32>; 2]]) -> usize {
    assignments
        .iter()
        .filter(|[x, y]| contains(x, y) || contains(y, x))
        .count()
}

pub fn part2(assignments: &[[RangeInclusive<u32>; 2]]) -> usize {
    assignments.iter().filter(|[x, y]| overlap(x, y)).count()
}
