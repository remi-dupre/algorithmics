use std::iter;

use anyhow::{Context, Result};

use crate::util::{Direction, ALL_DIRECTIONS};

type Tree = u8;

pub fn parse(input: &str) -> Result<Vec<Vec<Tree>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .context("could not parse tree height")
                })
                .collect()
        })
        .collect()
}

pub fn iter_map(
    init: (usize, usize),
    step: (isize, isize),
    map: &[Vec<u8>],
) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
    let (dx, dy) = step;
    let mut x = init.0 as isize;
    let mut y = init.1 as isize;

    iter::from_fn(move || {
        let res_x: usize = x.try_into().ok()?;
        let res_y: usize = y.try_into().ok()?;
        let row = map.get(res_y)?;
        let cell = *row.get(res_x)?;
        x += dx;
        y += dy;
        Some((res_x, res_y, cell))
    })
}

pub fn part1(map: &[Vec<u8>]) -> usize {
    let width = map[0].len();
    let height = map.len();
    type StartMapping = dyn Fn(usize) -> (usize, usize);

    let sides = [
        (width, Direction::Right, &(|k| (0usize, k)) as &StartMapping),
        (width, Direction::Left, &(|k| (height - 1, k)) as &_),
        (height, Direction::Down, &(|k| (k, 0usize)) as &_),
        (height, Direction::Up, &(|k| (k, width - 1)) as &_),
    ];

    let mut count_visible = 0;
    let mut seen = vec![vec![false; width]; height];

    for (max, dir, start) in sides {
        for k in 0..max {
            let mut max_visibility = 0;

            for (i, j, val) in iter_map(start(k), dir.into(), map) {
                if val >= max_visibility {
                    let cell = &mut seen[i][j];

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

pub fn part2(map: &[Vec<u8>]) -> Option<usize> {
    let height = map.len();
    let width = map[0].len();
    let cells = (0..width).flat_map(|x| (0..height).map(move |y| (x, y)));

    cells
        .map(|(x, y)| {
            let cell_val = map[y][x];

            ALL_DIRECTIONS
                .into_iter()
                .map(|dir| {
                    let mut count = 0;

                    for (_, _, val) in iter_map((x, y), dir.into(), map).skip(1) {
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
