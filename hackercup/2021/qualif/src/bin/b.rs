use testcase_derive::{hackercup, TestCase};

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    n: usize,
    #[testcase(lines = "n")]
    grid: Vec<String>,
}

// ---
// --- Cell
// ---

#[derive(Eq, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    fn from_byte(b: u8) -> Result<Self, String> {
        match b {
            b'.' => Ok(Self::Empty),
            b'X' => Ok(Self::X),
            b'O' => Ok(Self::O),
            _ => Err(format!("Unknown cell type '{}'", b as char)),
        }
    }
}

// ---
// --- Game
// ---

struct Game(Vec<Vec<Cell>>);

impl Game {
    fn read(input: Vec<String>) -> Result<Self, String> {
        let cells: Vec<_> = input
            .into_iter()
            .map(|line| line.bytes().map(Cell::from_byte).collect())
            .collect::<Result<_, _>>()?;

        Ok(Self(cells))
    }

    fn count_line(&self, line: usize, val: Cell) -> usize {
        self.0[line].iter().filter(|c| **c == val).count()
    }

    fn count_col(&self, col: usize, val: Cell) -> usize {
        self.0.iter().filter(|line| line[col] == val).count()
    }

    fn wins(&self) -> bool {
        let n = self.0.len();

        (0..n).any(|l| self.count_line(l, Cell::X) == n)
            || (0..n).any(|c| self.count_col(c, Cell::X) == n)
    }

    fn count_single_move_wins(&self) -> usize {
        let n = self.0.len();

        (0..n)
            .flat_map(|l| (0..n).map(move |c| (c, l)))
            .filter(|&(c, l)| {
                self.0[l][c] == Cell::Empty
                    && (self.count_line(l, Cell::X) == n - 1 || self.count_col(c, Cell::X) == n - 1)
            })
            .count()
    }

    fn count_win_moves(&self, add: usize) -> usize {
        let n = self.0.len();

        if add == 0 {
            if self.wins() {
                return 1;
            } else {
                return 0;
            }
        }

        if add == 1 {
            return self.count_single_move_wins();
        }

        let line_wins = (0..n)
            .filter(|&l| self.count_line(l, Cell::Empty) == add && self.count_line(l, Cell::O) == 0)
            .count();

        let col_wins = (0..n)
            .filter(|&c| self.count_col(c, Cell::Empty) == add && self.count_col(c, Cell::O) == 0)
            .count();

        line_wins + col_wins
    }
}

#[hackercup(input = "../../data/b/simple.in", output = "../../data/b/simple.out")]
fn solve(input: Input) -> String {
    let game = Game::read(input.grid).expect("Invalid grid");

    let result = (0..=input.n)
        .map(|nb_moves| (nb_moves, game.count_win_moves(nb_moves)))
        .find(|(_, solutions)| *solutions > 0);

    if let Some((nb_moves, solutions)) = result {
        format!("{} {}", nb_moves, solutions)
    } else {
        "Impossible".to_string()
    }
}
