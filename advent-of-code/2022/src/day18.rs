use std::ops::Range;

use anyhow::{Context, Result};
use fxhash::FxHashSet;

pub type Coord = [i8; 3];

fn neighbours([x, y, z]: Coord) -> impl Iterator<Item = Coord> {
    [
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, -1, 0],
        [0, 0, 1],
        [0, 0, -1],
    ]
    .into_iter()
    .map(move |[dx, dy, dz]| [x - dx, y - dy, z - dz])
}

pub fn parse(input: &str) -> Result<FxHashSet<Coord>> {
    input
        .lines()
        .map(|input| {
            let mut coords = input.splitn(3, ',');

            let x = coords
                .next()
                .context("missing 'x' coord")?
                .parse()
                .context("invalid 'x' coord")?;

            let y = coords
                .next()
                .context("missing 'y' coord")?
                .parse()
                .context("invalid 'y' coord")?;

            let z = coords
                .next()
                .context("missing 'z' coord")?
                .parse()
                .context("invalid 'z' coord")?;

            Ok([x, y, z])
        })
        .collect()
}

fn extend_range_for(range: &mut Range<i8>, val: i8) {
    if range.start > val {
        range.start = val;
    }

    if range.end <= val {
        range.end = val + 1
    }
}

fn bounds(coords: &FxHashSet<Coord>) -> [Range<i8>; 3] {
    #[allow(clippy::reversed_empty_ranges)]
    let [mut rng_x, mut rng_y, mut rng_z] = std::array::from_fn(|_| i8::MAX..i8::MIN);

    for &[x, y, z] in coords {
        extend_range_for(&mut rng_x, x);
        extend_range_for(&mut rng_y, y);
        extend_range_for(&mut rng_z, z);
    }

    [rng_x, rng_y, rng_z]
}

fn exterior(coords: &FxHashSet<Coord>) -> FxHashSet<Coord> {
    let bounds = bounds(coords).map(|rng| (rng.start - 1)..(rng.end + 1));
    let start = bounds.clone().map(|rng| rng.start);

    let in_bounds = move |coord: [i8; 3]| -> bool {
        bounds
            .iter()
            .zip(coord)
            .all(|(rng, coord)| rng.contains(&coord))
    };

    let mut todo = vec![start];
    let mut exterior: FxHashSet<_> = [start].into_iter().collect();

    while let Some(curr) = todo.pop() {
        for neighbour in neighbours(curr) {
            if in_bounds(neighbour)
                && !coords.contains(&neighbour)
                && !exterior.contains(&neighbour)
            {
                todo.push(neighbour);
                exterior.insert(neighbour);
            }
        }
    }

    exterior
}

pub fn part1(coords: &FxHashSet<Coord>) -> usize {
    coords
        .iter()
        .flat_map(|coord| neighbours(*coord))
        .filter(|neighbour| !coords.contains(neighbour))
        .count()
}

pub fn part2(coords: &FxHashSet<Coord>) -> usize {
    let exterior = exterior(coords);

    coords
        .iter()
        .flat_map(|coord| neighbours(*coord))
        .filter(|neighbour| exterior.contains(neighbour))
        .count()
}
