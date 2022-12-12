use std::collections::VecDeque;
use std::mem;

use anyhow::{bail, Context, Result};

type Cell = u8;

const CELL_START: Cell = b'S';
const CELL_END: Cell = b'E';

fn cell_val(cell: Cell) -> u8 {
    match cell {
        b'S' => b'a',
        b'E' => b'z',
        _ => cell,
    }
}

pub struct Map<'i> {
    cells: Vec<&'i [Cell]>,
    width: usize,
    heigth: usize,
}

impl<'i> Map<'i> {
    fn new(cells: Vec<&'i [Cell]>) -> Self {
        let heigth = cells.len();
        let width = cells.get(0).unwrap_or(&[].as_slice()).len();

        Self {
            cells,
            width,
            heigth,
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<Cell> {
        self.cells.get(y)?.get(x).copied()
    }

    fn find_pos(
        &self,
        pred: &'i impl Fn(Cell) -> bool,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.cells.iter().enumerate().flat_map(move |(y, line)| {
            line.iter()
                .enumerate()
                .filter(move |(_, b)| pred(**b))
                .map(move |(x, _)| (x, y))
        })
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        [
            {
                if x + 1 < self.width {
                    Some((x + 1, y))
                } else {
                    None
                }
            },
            {
                if y + 1 < self.heigth {
                    Some((x, y + 1))
                } else {
                    None
                }
            },
            x.checked_sub(1).map(|nx| (nx, y)),
            y.checked_sub(1).map(|ny| (x, ny)),
        ]
        .into_iter()
        .flatten()
    }
}

pub fn parse(input: &[u8]) -> Map<'_> {
    Map::new(
        input
            .split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
            .collect(),
    )
}

pub fn dist_to_end(map: &Map<'_>, start: impl IntoIterator<Item = (usize, usize)>) -> Result<u64> {
    let mut todo: VecDeque<_> = start
        .into_iter()
        .map(|pos| {
            let cell = map.get(pos).context("could not get init val")?;
            Ok((0, pos, cell))
        })
        .collect::<Result<_>>()?;

    let mut seen = vec![vec![false; map.width]; map.heigth];

    while let Some((dist, pos, cell)) = todo.pop_front() {
        for n_pos in map.neighbours(pos) {
            let n_cell = map.get(n_pos).context("could not get neighbour value")?;

            if cell_val(n_cell) <= cell_val(cell) + 1
                && !mem::replace(&mut seen[n_pos.1][n_pos.0], true)
            {
                if n_cell == CELL_END {
                    return Ok(dist + 1);
                }

                todo.push_back((dist + 1, n_pos, n_cell));
            }
        }
    }

    bail!("could not reach end")
}

pub fn part1(map: &Map<'_>) -> Result<u64> {
    let start = map
        .find_pos(&|cell| cell == CELL_START)
        .next()
        .context("could not find start position")?;

    dist_to_end(map, [start])
}

pub fn part2(map: &Map<'_>) -> Result<u64> {
    let start: Vec<_> = map.find_pos(&|cell| cell_val(cell) == b'a').collect();
    dist_to_end(map, start)
}
