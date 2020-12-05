pub fn generator(input: &str) -> Map {
    let width = input.lines().next().map(str::len).unwrap_or(0);
    let data = input.lines().flat_map(|line| {
        line.chars().map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Asteroid,
            _ => panic!("invalid cell `{}`", c),
        })
    });
    Map::new(width, data)
}

pub fn part_1(map: &Map) -> usize {
    map.asteroids()
        .map(|cand| {
            map.asteroids()
                .filter(|&other| other != cand)
                .filter(|&other| {
                    !map.asteroids().any(|inter| {
                        inter.0 != cand.0
                            && inter.1 != cand.1
                            && (other.0 - cand.0) % (inter.0 - cand.0) == 0
                            && (other.1 - cand.1) % (inter.1 - cand.1) == 0
                            && (other.0 - cand.0) / (inter.0 - cand.0)
                                == (other.1 - cand.1) / (inter.1 - cand.1)
                    })
                })
                .count()
        })
        .max()
        .expect("no asteroid")
}

// ---
// --- Structs
// ---

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Asteroid,
}

pub struct Map {
    data: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(width: usize, data: impl IntoIterator<Item = Cell>) -> Self {
        let data: Vec<_> = data.into_iter().collect();
        let height = data.len() / width;
        assert_eq!(width * height, data.len(), "bad shape for Map");

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

    fn asteroids(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.width).flat_map(move |x| {
            (0..self.height)
                .map(move |y| (x, y))
                .filter(move |&(x, y)| self.get(x, y) == Some(Cell::Asteroid))
        })
    }
}
