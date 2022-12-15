use anyhow::{Context, Result};

use crate::util::matrix::{Matrix, MatrixPtr};
use crate::util::min_max;

const CENTER_X: usize = 500;

type Coord = (usize, usize);

pub struct Map {
    min_x: usize,
    min_y: usize,
    cells: Matrix<bool>,
}

impl Map {
    fn new(x_bounds: Coord, y_bounds: Coord) -> Self {
        let width = x_bounds.1 - x_bounds.0;
        let height = y_bounds.1 - y_bounds.0;
        let cells = Matrix::new(width, height, false);

        Self {
            min_x: x_bounds.0,
            min_y: y_bounds.0,
            cells,
        }
    }

    fn insert(&mut self, (x, y): Coord) {
        self.cells[(x - self.min_x, y - self.min_y)] = true;
    }

    fn get_ptr(&mut self, (x, y): Coord) -> Option<MatrixPtr<'_, bool>> {
        self.cells.get_ptr_mut((x - self.min_x, y - self.min_y))
    }
}

pub fn parse(input: &str) -> Result<Vec<Vec<Coord>>> {
    let paths = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|raw_rock| {
                    let (x, y) = raw_rock
                        .split_once(',')
                        .context("missing comma separator for rock")?;

                    let x = x.parse().context("invalid 'x' coordinate")?;
                    let y = y.parse().context("invalid 'y' coordinate")?;
                    Ok((x, y))
                })
                .collect()
        })
        .collect::<Result<_>>()?;

    Ok(paths)
}

fn walk_fall<'m>(
    mut ptr: MatrixPtr<'m, bool>,
    counter: &mut u64,
    has_floor: bool,
) -> Option<MatrixPtr<'m, bool>> {
    match ptr.get().copied() {
        Some(false) => {}
        Some(true) => return Some(ptr),
        None => {
            if has_floor {
                return Some(ptr);
            } else {
                return None;
            }
        }
    }

    // Try to fill bellow, exit if end is reached
    ptr = walk_fall(ptr.add_rows(1), counter, has_floor)?;
    ptr = walk_fall(ptr.sub_cols(1), counter, has_floor)?;
    ptr = walk_fall(ptr.add_cols(2), counter, has_floor)?;

    // Reset pointer, fill cell and exit
    ptr = ptr.sub_rows(1).sub_cols(1);
    ptr.set(true);
    *counter += 1;
    Some(ptr)
}

fn simulate_fall(path: &[Vec<Coord>], start: Coord, has_floor: bool) -> u64 {
    let Some(y_max) = path_coords(path).map(|(_, y)| y).max() else {
        return 0
    };

    let mut map = Map::new((CENTER_X - y_max - 2, CENTER_X + y_max + 2), (0, y_max + 2));

    for coord in path_coords(path) {
        map.insert(coord);
    }

    let mut counter = 0;
    let ptr = map.get_ptr(start).expect("invalid start pos");
    walk_fall(ptr, &mut counter, has_floor);
    counter
}

fn path_coords(paths: &[Vec<Coord>]) -> impl Iterator<Item = Coord> + '_ {
    paths.iter().flat_map(|segment| {
        segment.array_windows().flat_map(|&[start, end]| {
            let (x_min, x_max) = min_max(start.0, end.0);
            let (y_min, y_max) = min_max(start.1, end.1);
            (x_min..=x_max).flat_map(move |x| (y_min..=y_max).map(move |y| (x, y)))
        })
    })
}

pub fn part1(path: &[Vec<Coord>]) -> u64 {
    simulate_fall(path, (CENTER_X, 0), false)
}

pub fn part2(path: &[Vec<Coord>]) -> u64 {
    simulate_fall(path, (CENTER_X, 0), true)
}
