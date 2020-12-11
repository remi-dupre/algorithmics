use std::fmt;

use crate::utils::{Matrix, SignedAdd};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn generator(input: &str) -> Result<Matrix<Cell>, String> {
    let width = input.lines().next().map(str::len).unwrap_or(0);
    let cells = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '.' => Ok(Cell::Empty),
                'L' => Ok(Cell::Seat),
                '#' => Ok(Cell::Occupied),
                _ => Err(format!("unknown cell `{}`", c)),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Matrix::new(cells, width))
}

fn apply_until_stable(
    mut map: Matrix<Cell>,
    swap: impl Fn(&Matrix<Cell>, usize, usize) -> Option<Cell>,
) -> Matrix<Cell> {
    loop {
        let swaps: Vec<_> = map
            .iter_pos()
            .filter_map(|(x, y)| Some(((x, y), swap(&map, x, y)?)))
            .collect();

        if swaps.is_empty() {
            break;
        }

        for ((x, y), cell) in swaps {
            map[(x, y)] = cell;
        }
    }

    map
}

pub fn part_1(map: &Matrix<Cell>) -> usize {
    fn swap(map: &Matrix<Cell>, x: usize, y: usize) -> Option<Cell> {
        let occ_adj = DIRECTIONS
            .iter()
            .filter_map(|&(dx, dy)| map.get(x.signed_add(dx)?, y.signed_add(dy)?))
            .filter(|&cell| *cell == Cell::Occupied)
            .count();

        match (map[(x, y)], occ_adj) {
            (Cell::Seat, 0) => Some(Cell::Occupied),
            (Cell::Occupied, 4..=8) => Some(Cell::Seat),
            _ => None,
        }
    }

    apply_until_stable(map.clone(), swap)
        .values()
        .filter(|&&cell| cell == Cell::Occupied)
        .count()
}

pub fn part_2(map: &Matrix<Cell>) -> usize {
    fn proj(map: &Matrix<Cell>, (x, y): (usize, usize), (dx, dy): (isize, isize)) -> Option<Cell> {
        let mut x = x.signed_add(dx)?;
        let mut y = y.signed_add(dy)?;

        while let Some(cell) = map.get(x, y) {
            match cell {
                Cell::Empty => {
                    x = x.signed_add(dx)?;
                    y = y.signed_add(dy)?;
                }
                other => return Some(*other),
            }
        }

        None
    }

    fn swap(map: &Matrix<Cell>, x: usize, y: usize) -> Option<Cell> {
        let occ_adj = DIRECTIONS
            .iter()
            .filter(|&&(dx, dy)| proj(map, (x, y), (dx, dy)) == Some(Cell::Occupied))
            .count();

        match (map[(x, y)], occ_adj) {
            (Cell::Seat, 0) => Some(Cell::Occupied),
            (Cell::Occupied, 5..=8) => Some(Cell::Seat),
            _ => None,
        }
    }

    apply_until_stable(map.clone(), swap)
        .values()
        .filter(|&&cell| cell == Cell::Occupied)
        .count()
}

// ---
// --- Structs
// ---

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Seat,
    Occupied,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Occupied => '#',
                Self::Seat => 'L',
                Self::Empty => '.',
            }
        )
    }
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day11::*;

    const EXAMPLE: &str = crate::lines! {
        "L.LL.LL.LL"
        "LLLLLLL.LL"
        "L.L.L..L.."
        "LLLL.LL.LL"
        "L.LL.LL.LL"
        "L.LLLLL.LL"
        "..L.L....."
        "LLLLLLLLLL"
        "L.LLLLLL.L"
        "L.LLLLL.LL"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(37, part_1(&generator(EXAMPLE).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(26, part_2(&generator(EXAMPLE).unwrap()));
    }
}
