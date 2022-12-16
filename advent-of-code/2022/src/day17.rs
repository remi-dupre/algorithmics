use anyhow::{bail, Result};
use fxhash::FxHashMap;

const PIECES_HEIGHT: usize = 4;

type Piece = [u8; PIECES_HEIGHT];

const PIECES: [Piece; 5] = [
    [0b_0001_1110, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000], // —
    [0b_0000_1000, 0b_0001_1100, 0b_0000_1000, 0b_0000_0000], // +
    [0b_0001_1100, 0b_0000_0100, 0b_0000_0100, 0b_0000_0000], // ⅃
    [0b_0001_0000, 0b_0001_0000, 0b_0001_0000, 0b_0001_0000], // |
    [0b_0001_1000, 0b_0001_1000, 0b_0000_0000, 0b_0000_0000], // ▅
];

fn piece_height(piece: Piece) -> usize {
    piece.iter().take_while(|x| **x != 0).count()
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn apply(&self, piece: Piece) -> Piece {
        match self {
            Direction::Left => {
                if piece.iter().any(|&row| row & 0b_0100_0000 != 0) {
                    return piece;
                }

                piece.map(|row| row << 1)
            }
            Direction::Right => {
                if piece.iter().any(|&row| row & 0b_0000_0001 != 0) {
                    return piece;
                }

                piece.map(|row| row >> 1)
            }
        }
    }
}

fn conflicts(piece: &Piece, state: &[u8]) -> bool {
    piece.iter().zip(state).any(|(&x, &y)| x & !y != x)
}

fn simulate_fall<'d>(
    state: &mut Vec<u8>,
    mut piece: Piece,
    mut directions: impl Iterator<Item = &'d Direction>,
) -> Option<usize> {
    // The first 4 horizontal moves can't conflict
    for direction in directions.by_ref().take(4) {
        piece = direction.apply(piece);
    }

    let mut v_pos = state.len();

    loop {
        // Move down if possible
        let is_blocked = v_pos == 0 || conflicts(&piece, &state[(v_pos - 1)..]);

        if is_blocked {
            if v_pos + piece_height(piece) > state.len() {
                state.resize(v_pos + piece_height(piece), 0b_1000_0000)
            }

            break (piece.iter().zip(&mut state[v_pos..]))
                .enumerate()
                .filter_map(|(i, (&row_piece, row_state))| {
                    *row_state |= row_piece;

                    if *row_state == 0b_1111_1111 {
                        Some(v_pos + i)
                    } else {
                        None
                    }
                })
                .last();
        }

        // Move horizontaly
        let direction = directions.next().unwrap();
        v_pos -= 1;

        piece = {
            let moved_piece = direction.apply(piece);

            if conflicts(&moved_piece, &state[v_pos..]) {
                piece
            } else {
                moved_piece
            }
        };
    }
}

pub fn parse(input: &str) -> Result<Vec<Direction>> {
    input
        .chars()
        .map(|c| match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => bail!("unexpected direction {c}"),
        })
        .collect()
}

pub fn part1(directions: &[Direction]) -> usize {
    const TOTAL_STEPS: usize = 2022;
    let mut state = Vec::new();
    let mut directions = directions.iter().cycle();

    for &piece in PIECES.iter().cycle().take(TOTAL_STEPS) {
        simulate_fall(&mut state, piece, &mut directions);
    }

    state.len()
}

pub fn part2(directions: &[Direction]) -> usize {
    const TOTAL_STEPS: usize = 1_000_000_000_000;
    const MASK_SIZE: usize = 8;

    let mut consumed_directions = 0usize;
    let mut directions_iter = directions.iter().cycle();

    let mut state = Vec::new();
    let mut step = PIECES.len();
    let mut history = FxHashMap::default();
    let mut last_filled = None;

    let (init_step, init_height) = loop {
        let piece = PIECES[step % PIECES.len()];
        step += 1;

        if let Some(new_last_filled) = simulate_fall(
            &mut state,
            piece,
            (&mut directions_iter).inspect(|_| consumed_directions += 1),
        ) {
            last_filled = Some(new_last_filled);
        }

        let last_filled = last_filled.unwrap_or(state.len().saturating_sub(MASK_SIZE));

        if state.len() - last_filled <= MASK_SIZE {
            let current_footprint = (
                consumed_directions % directions.len(),
                (step - 1) % PIECES.len(),
                state[last_filled..].to_vec(),
            );

            if let Some((init_step, init_height)) = history.get(&current_footprint) {
                break (init_step, init_height);
            }

            history.insert(current_footprint, (step, state.len()));
        }
    };

    let repetition_height = state.len() - init_height;
    let repetition_steps = step - init_step;
    let repetition_count = (TOTAL_STEPS - init_step) / repetition_steps;

    let end_steps = 5 + // ???
        TOTAL_STEPS
        - init_step
        - repetition_steps * repetition_count;

    for &piece in (PIECES.iter().cycle())
        .skip(step % PIECES.len())
        .take(end_steps)
    {
        simulate_fall(&mut state, piece, &mut directions_iter);
    }

    let end_height = state.len() - repetition_height - init_height;
    init_height + repetition_height * repetition_count + end_height
}
