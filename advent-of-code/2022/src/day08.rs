use std::iter;

use anyhow::{Context, Result};

use crate::util::matrix::Matrix;
use crate::util::{Direction, ALL_DIRECTIONS};

type Tree = u8;

pub fn parse(input: &str) -> Result<Matrix<Tree>> {
    let width = input.lines().next().map(|line| line.len()).unwrap_or(0);
    let height = input.lines().count();

    let cells = input.lines().flat_map(|line| {
        line.chars().map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .context("could not parse tree height")
        })
    });

    Matrix::try_from_iter(width, height, cells)
}

pub fn iter_map(
    init: (usize, usize),
    step: (isize, isize),
    map: &Matrix<Tree>,
) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    let (dx, dy) = step;
    let mut x = init.0 as isize;
    let mut y = init.1 as isize;

    iter::from_fn(move || {
        let res_x: usize = x.try_into().ok()?;
        let res_y: usize = y.try_into().ok()?;
        let cell = *map.get((res_x, res_y))?;
        x += dx;
        y += dy;
        Some(((res_x, res_y), cell))
    })
}

pub fn part1(map: &Matrix<Tree>) -> usize {
    let width = map.width();
    let height = map.height();
    type StartMapping = dyn Fn(usize) -> (usize, usize);

    let sides = [
        (width, Direction::Right, &(|k| (0usize, k)) as &StartMapping),
        (width, Direction::Left, &(|k| (height - 1, k)) as &_),
        (height, Direction::Down, &(|k| (k, 0usize)) as &_),
        (height, Direction::Up, &(|k| (k, width - 1)) as &_),
    ];

    let mut count_visible = 0;
    let mut seen = Matrix::new(width, height, false);

    for (max, dir, start) in sides {
        for k in 0..max {
            let mut max_visibility = 0;

            for (pos, val) in iter_map(start(k), dir.into(), map) {
                if val >= max_visibility {
                    let cell = &mut seen[pos];

                    if !*cell {
                        count_visible += 1;
                        *cell = true;
                    }

                    max_visibility = val + 1;
                }
            }
        }
    }

    count_visible
}

pub fn part2(map: &Matrix<Tree>) -> Option<usize> {
    map.iter()
        .map(|(pos, &cell_val)| {
            ALL_DIRECTIONS
                .into_iter()
                .map(|dir| {
                    let mut count = 0;

                    for (_, val) in iter_map(pos, dir.into(), map).skip(1) {
                        count += 1;

                        if val >= cell_val {
                            break;
                        }
                    }

                    count
                })
                .product()
        })
        .max()
}
