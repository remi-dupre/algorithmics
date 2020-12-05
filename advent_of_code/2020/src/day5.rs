use std::convert::TryInto;

const ROWS: u16 = 128;
const COLS: u16 = 8;
const MAX_ID: u16 = ROWS * COLS;

#[derive(Copy, Clone, Debug)]
pub enum Partition {
    Front,
    Back,
    Left,
    Right,
}

pub fn generator(input: &str) -> Vec<[Partition; 10]> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' => Partition::Front,
                    'B' => Partition::Back,
                    'L' => Partition::Left,
                    'R' => Partition::Right,
                    _ => panic!("invalid character `{}`", c),
                })
                .collect::<Vec<_>>()
                .try_into()
                .expect("seat assignment must be 10 character long")
        })
        .collect()
}

fn seat_id(seat: &[Partition; 10]) -> u16 {
    let mut row = 0..ROWS;
    let mut col = 0..COLS;

    for &partition in seat.iter() {
        match partition {
            Partition::Front => row.end = (row.start + row.end + 1) / 2,
            Partition::Back => row.start = (row.start + row.end + 1) / 2,
            Partition::Left => col.end = (col.start + col.end + 1) / 2,
            Partition::Right => col.start = (col.start + col.end + 1) / 2,
        }
    }

    assert_eq!(row.start + 1, row.end, "invalid partitioning for rows");
    assert_eq!(col.start + 1, col.end, "invalid partitioning for cols");
    row.start * 8 + col.start
}

pub fn part_1(seats: &[[Partition; 10]]) -> u16 {
    seats.iter().map(seat_id).max().expect("empty input")
}

pub fn part_2(seats: &[[Partition; 10]]) -> u16 {
    let mut is_taken = [false; MAX_ID as usize];

    for id in seats.iter().map(seat_id) {
        is_taken[usize::from(id)] = true;
    }

    is_taken
        .array_windows()
        .enumerate()
        .filter(|(_, &[left, seat, right])| left && !seat && right)
        .map(|(i, _)| (i + 1).try_into().expect("invalid seat id"))
        .next()
        .expect("no feasible seat")
}
