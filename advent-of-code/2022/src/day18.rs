use std::ops::Range;

use anyhow::{Context, Result};

pub type Coord = [i8; 3];

pub struct Map {
    bounds: [Range<i8>; 3],
    shape: [usize; 3],
    cells: Vec<bool>,
}

impl Map {
    fn new(bounds: [Range<i8>; 3]) -> Self {
        let shape = bounds
            .clone()
            .map(|rng| (rng.end - rng.start).try_into().expect("invalid shape"));

        let cells_len = shape.iter().product();
        let cells = vec![false; cells_len];

        Self {
            bounds,
            shape,
            cells,
        }
    }

    fn index_for(&self, [x, y, z]: [i8; 3]) -> usize {
        let [dx, dy, dz] = self.bounds.clone().map(|rng| rng.start);
        let [x, y, z] = [x - dx, y - dy, z - dz].map(|val| val as usize);
        x + self.shape[0] * (y + self.shape[1] * z)
    }

    fn contains(&self, pos: [i8; 3]) -> bool {
        let idx = self.index_for(pos);
        self.cells[idx]
    }

    fn insert(&mut self, pos: [i8; 3]) {
        let idx = self.index_for(pos);
        self.cells[idx] = true;
    }
}

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

fn bounds(coords: &[Coord]) -> [Range<i8>; 3] {
    fn extend_range_for(range: &mut Range<i8>, val: i8) {
        if range.start > val {
            range.start = val;
        }

        if range.end <= val {
            range.end = val + 1
        }
    }

    #[allow(clippy::reversed_empty_ranges)]
    let [mut rng_x, mut rng_y, mut rng_z] = std::array::from_fn(|_| i8::MAX..i8::MIN);

    for &[x, y, z] in coords {
        extend_range_for(&mut rng_x, x);
        extend_range_for(&mut rng_y, y);
        extend_range_for(&mut rng_z, z);
    }

    [rng_x, rng_y, rng_z]
}

fn build_map(cells: &[Coord]) -> Map {
    let bounds = bounds(cells).map(|rng| (rng.start - 1)..(rng.end + 1));
    let mut map = Map::new(bounds);

    for &pos in cells {
        map.insert(pos);
    }

    map
}

fn build_exterior(map: &Map) -> Map {
    let start = map.bounds.clone().map(|rng| rng.start);

    let in_bounds = move |coord: [i8; 3]| -> bool {
        map.bounds
            .iter()
            .zip(coord)
            .all(|(rng, coord)| rng.contains(&coord))
    };

    let mut todo = vec![start];
    let mut exterior = Map::new(map.bounds.clone());
    exterior.insert(start);

    while let Some(curr) = todo.pop() {
        for neighbour in neighbours(curr) {
            if in_bounds(neighbour) && !map.contains(neighbour) && !exterior.contains(neighbour) {
                todo.push(neighbour);
                exterior.insert(neighbour);
            }
        }
    }

    exterior
}

pub fn parse(input: &str) -> Result<Vec<Coord>> {
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

pub fn part1(coords: &[Coord]) -> usize {
    let map = build_map(coords);

    coords
        .iter()
        .flat_map(|coord| neighbours(*coord))
        .filter(|&neighbour| !map.contains(neighbour))
        .count()
}

pub fn part2(coords: &[Coord]) -> usize {
    let map = build_map(coords);
    let exterior = build_exterior(&map);

    coords
        .iter()
        .flat_map(|coord| neighbours(*coord))
        .filter(|&neighbour| exterior.contains(neighbour))
        .count()
}
