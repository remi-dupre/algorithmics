pub fn generator(input: &str) -> Grid {
    let width = input.lines().next().map(str::len).unwrap_or(0);
    let data = input.lines().flat_map(|line| {
        line.chars().map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Tree,
            _ => panic!("invalid cell `{}`", c),
        })
    });
    Grid::new(width, data)
}

fn count_trees(grid: &Grid, cells: impl IntoIterator<Item = (usize, usize)>) -> usize {
    cells
        .into_iter()
        .filter(|&(x, y)| grid.get(x, y).expect("invalid position") == Cell::Tree)
        .count()
}

pub fn part_1(grid: &Grid) -> usize {
    count_trees(grid, grid.slope(3, 1))
}

pub fn part_2(grid: &Grid) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(x, y)| count_trees(grid, grid.slope(x, y)))
        .product()
}

// ---
// --- Structs
// ---

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Tree,
}

pub struct Grid {
    data: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, data: impl IntoIterator<Item = Cell>) -> Self {
        let data: Vec<_> = data.into_iter().collect();
        let height = data.len() / width;
        assert_eq!(width * height, data.len(), "bad shape for Grid");

        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        let x = x % self.width;
        self.data.get(x + y * self.width).copied()
    }

    fn slope(&self, dx: usize, dy: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height / dy).map(move |it| (it * dx, it * dy))
    }
}
