use std::fmt;
use testcase_derive::{hackercup, TestCase};

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    _r: usize,
    _c: usize,
    #[testcase(lines = "_r")]
    grid: Vec<String>,
}

fn neighbours(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(move |(dx, dy)| (x + dx, y + dy))
}

// ---
// --- Cell
// ---

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Tree,
    Rock,
}

impl Cell {
    fn from_byte(b: u8) -> Result<Self, String> {
        match b {
            b'.' => Ok(Self::Empty),
            b'^' => Ok(Self::Tree),
            b'#' => Ok(Self::Rock),
            _ => Err(format!("Unknown cell type '{}'", b as char)),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Tree => write!(f, "^"),
            Cell::Rock => write!(f, "#"),
        }
    }
}

// ---
// --- Grid
// ---

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn read(input: Vec<String>) -> Result<Self, String> {
        let cells: Vec<Vec<Cell>> = input
            .into_iter()
            .map(|line| line.bytes().map(Cell::from_byte).collect())
            .collect::<Result<_, _>>()?;

        Ok(Self { cells })
    }

    fn width(&self) -> isize {
        self.cells[0].len() as isize
    }

    fn height(&self) -> isize {
        self.cells.len() as isize
    }

    fn get(&self, x: isize, y: isize) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None;
        }

        self.cells.get(y as usize)?.get(x as usize).copied()
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut Cell> {
        if x < 0 || y < 0 {
            return None;
        }

        self.cells.get_mut(y as usize)?.get_mut(x as usize)
    }

    fn is_lonely(&self, x: isize, y: isize) -> bool {
        let growable_neighbours = neighbours(x, y)
            .filter(|&(nx, ny)| matches!(self.get(nx, ny), Some(Cell::Empty | Cell::Tree)))
            .count();

        growable_neighbours < 2
    }

    fn try_grow(&self) -> Option<Self> {
        let mut soluce = self.clone();

        let mut todo: Vec<_> = (0..self.width())
            .flat_map(|x| (0..self.height()).map(move |y| (x, y)))
            .collect();

        while let Some((x, y)) = todo.pop() {
            if soluce.is_lonely(x, y) {
                match soluce.get(x, y) {
                    None | Some(Cell::Rock) => {}
                    Some(Cell::Tree) => return None,
                    Some(Cell::Empty) => {
                        *soluce.get_mut(x, y).unwrap() = Cell::Rock;
                        todo.extend(neighbours(x, y));
                    }
                }
            }
        }

        // Cleanup construction data
        for x in 0..self.width() {
            for y in 0..self.height() {
                if soluce.get(x, y).unwrap() == Cell::Empty {
                    *soluce.get_mut(x, y).unwrap() = Cell::Tree;
                } else if soluce.get(x, y).unwrap() == Cell::Rock
                    && self.get(x, y).unwrap() == Cell::Empty
                {
                    *soluce.get_mut(x, y).unwrap() = Cell::Empty;
                }
            }
        }

        Some(soluce)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[hackercup(input = "../../data/b1/simple.in", output = "../../data/b1/simple.out")]
fn solve(input: Input) -> String {
    let grid = Grid::read(input.grid).expect("invalid input");

    if let Some(result) = grid.try_grow() {
        format!("Possible\n{result}")
    } else {
        "Impossible".to_string()
    }
}
