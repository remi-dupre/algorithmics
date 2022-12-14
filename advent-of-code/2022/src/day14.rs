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

fn simulate_fall(map: &mut Map, pos: Coord) -> bool {
    let mut search_insert_pos = move || {
        let mut ptr = map.get_ptr(pos).expect("invalid start pos");

        loop {
            ptr.add_rows(1);

            if !*ptr.get()? {
                continue;
            }

            ptr.sub_cols(1);

            if !*ptr.get()? {
                continue;
            }

            ptr.add_cols(2);

            if !*ptr.get()? {
                continue;
            }

            ptr.sub_cols(1);
            ptr.sub_rows(1);
            break;
        }

        assert!(ptr.set(true));
        Some(())
    };

    search_insert_pos().is_some()
}

fn simulate_fall_with_floor(map: &mut Map, (x, y): Coord, y_max: usize) -> bool {
    let y_diff = y_max - y - 1;

    // Ensure that we can move `y_diff` steps to bottom-left or to bottom right.
    assert!(map.get_ptr((x - y_diff, y + y_diff)).is_some());
    assert!(map.get_ptr((x + y_diff, y + y_diff)).is_some());

    let mut ptr = map.get_ptr((x, y)).expect("invalid start pos");

    if *ptr.get().expect("invalid created pointer") {
        return false;
    }

    for _ in 0..y_diff {
        ptr.add_rows(1);

        // The pointer can only go one row down, which means that we never point to more than
        // row y_max as we do (y_max - y) iterations from y
        if !*unsafe { ptr.get_unchecked() } {
            continue;
        }

        ptr.sub_cols(1);

        // The pointer never goes left / right more than it goes down, we ensured that the
        // matrix is twice as wide as it is tall
        if !*unsafe { ptr.get_unchecked() } {
            continue;
        }

        ptr.add_cols(2);

        // We just moved left once so moving right twice cancels out to moving right only once
        if !*unsafe { ptr.get_unchecked() } {
            continue;
        }

        // Cancel previous move before returning
        ptr.sub_cols(1);
        ptr.sub_rows(1);
        break;
    }

    assert!(ptr.set(true));
    true
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
    let y_max = path_coords(path).map(|(_, y)| y).max().unwrap_or(0);

    let x_min = path_coords(path)
        .map(|(x, _)| x)
        .chain([CENTER_X])
        .min()
        .unwrap();

    let x_max = path_coords(path)
        .map(|(x, _)| x)
        .chain([CENTER_X])
        .max()
        .unwrap();

    let mut count = 0;
    let mut map = Map::new((x_min, x_max), (0, y_max + 1));

    for coord in path_coords(path) {
        map.insert(coord);
    }

    while simulate_fall(&mut map, (CENTER_X, 0)) {
        count += 1;
    }

    count
}

pub fn part2(path: &[Vec<Coord>]) -> u64 {
    let Some(y_max) = path_coords(path).map(|(_, y)| y).max() else {
        return 0
    };

    let mut count = 0;
    let mut map = Map::new((CENTER_X - y_max - 2, CENTER_X + y_max + 2), (0, y_max + 2));

    for coord in path_coords(path) {
        map.insert(coord);
    }

    while simulate_fall_with_floor(&mut map, (CENTER_X, 0), y_max + 2) {
        count += 1
    }

    count
}
