#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Down,
    Right,
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Up,
    Direction::Down,
    Direction::Right,
];

macro_rules! impl_direction_from_pair {
    ( $( $coord: ty ),* ) => {$(
        impl From<Direction> for ($coord, $coord) {
            fn from(val: Direction) -> Self {
                match val {
                    Direction::Left => (-1, 0),
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Right => (1, 0),
                }
            }
        }
    )*};
}

impl_direction_from_pair!(isize, i128, i64, i32, i16, i8);
