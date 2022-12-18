use std::iter;

use anyhow::{bail, Context, Result};
use rustc_hash::FxHashSet;

use crate::util::Direction;

pub fn parse(input: &str) -> Result<Vec<(Direction, usize)>> {
    input
        .lines()
        .map(|line| {
            let (dir, count) = line
                .split_once(' ')
                .context("could not find whitespace separator")?;

            let dir = match dir {
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                _ => bail!("invalid direction: '{dir}"),
            };

            let count = count.parse().context("could not parse count")?;
            Ok((dir, count))
        })
        .collect()
}

fn moves(input: &[(Direction, usize)]) -> impl Iterator<Item = Direction> + '_ {
    input
        .iter()
        .flat_map(|(dir, count)| iter::repeat(*dir).take(*count))
}

fn ensure_abs_ceil(head: (i32, i32), tail: &mut (i32, i32)) -> bool {
    let diff = i32::max((head.0 - tail.0).abs(), (head.1 - tail.1).abs());
    let legal = diff <= 1;

    if !legal {
        tail.0 = head.0 + (tail.0 - head.0) / diff;
        tail.1 = head.1 + (tail.1 - head.1) / diff;
    }

    !legal
}

fn simulate_rope<const K: usize>(
    mut rope: [(i32, i32); K],
    mut moves: impl Iterator<Item = Direction>,
) -> impl Iterator<Item = [(i32, i32); K]> {
    iter::from_fn(move || {
        let (dx, dy): (i32, i32) = moves.next()?.into();
        let head = rope.last_mut()?;
        head.0 += dx;
        head.1 += dy;

        for i in (1..K).rev() {
            if !ensure_abs_ceil(rope[i], &mut rope[i - 1]) {
                // Early exit if this knot didn't move
                break;
            }
        }

        Some(rope)
    })
}

pub fn part1(input: &[(Direction, usize)]) -> usize {
    let track = simulate_rope([(0, 0); 2], moves(input)).map(|[tail, ..]| tail);
    let track: FxHashSet<_> = iter::once((0, 0)).chain(track).collect();
    track.len()
}

pub fn part2(input: &[(Direction, usize)]) -> usize {
    let track = simulate_rope([(0, 0); 10], moves(input)).map(|[tail, ..]| tail);
    let track: FxHashSet<_> = iter::once((0, 0)).chain(track).collect();
    track.len()
}
