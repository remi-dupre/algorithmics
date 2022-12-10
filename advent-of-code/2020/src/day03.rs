use crate::utils::Matrix;
use std::convert::TryFrom;

pub fn generator(input: &str) -> Result<Matrix<Cell>, String> {
    input.parse()
}

fn slope(
    grid: &Matrix<Cell>,
    (dx, dy): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..grid.height() / dy).map(move |it| ((it * dx) % grid.width(), it * dy))
}

fn count_trees(grid: &Matrix<Cell>, cells: impl IntoIterator<Item = (usize, usize)>) -> usize {
    cells
        .into_iter()
        .filter(|&(x, y)| grid[(x, y)] == Cell::Tree)
        .count()
}

pub fn part_1(grid: &Matrix<Cell>) -> usize {
    count_trees(grid, slope(grid, (3, 1)))
}

pub fn part_2(grid: &Matrix<Cell>) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(x, y)| count_trees(grid, slope(grid, (x, y))))
        .product()
}

// ---
// --- Structs
// ---

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Tree,
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Tree),
            _ => Err(format!("invalid cell `{}`", value)),
        }
    }
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day03::*;

    const EXAMPLE: &str = crate::lines! {
        "..##.........##.........##.........##.........##.........##......."
        "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#.."
        ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#."
        "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#"
        ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#."
        "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##....."
        ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#"
        ".#........#.#........#.#........#.#........#.#........#.#........#"
        "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#..."
        "#...##....##...##....##...##....##...##....##...##....##...##....#"
        ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(7, part_1(&generator(EXAMPLE).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(336, part_2(&generator(EXAMPLE).unwrap()));
    }
}
