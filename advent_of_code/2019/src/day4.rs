use aoc_runner_derive::{aoc, aoc_generator};
use std::iter;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (u64, u64) {
    let mut passwords = input.split('-');
    (
        passwords.next().unwrap().parse().unwrap(),
        passwords.next().unwrap().parse().unwrap(),
    )
}

fn groups<T: Eq>(mut vals: impl Iterator<Item = T>) -> impl Iterator<Item = usize> {
    let mut item: Option<T> = vals.next();
    let mut count = 1;

    iter::from_fn(move || {
        if item.is_some() {
            loop {
                let new = vals.next();

                if new == item {
                    count += 1;
                } else {
                    item = new;
                    break;
                }
            }

            let res = count;
            count = 1;
            Some(res)
        } else {
            None
        }
    })
}

#[aoc(day4, part1)]
pub fn part1(input: &(u64, u64)) -> usize {
    (input.0..=input.1)
        .filter(|candidate| {
            let as_str = candidate.to_string();

            let is_sorted = as_str.as_bytes().windows(2).all(|pair| {
                if let [x, y] = pair {
                    x <= y
                } else {
                    unreachable!()
                }
            });

            let has_pair = groups(as_str.bytes()).any(|group_size| group_size >= 2);
            is_sorted && has_pair
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &(u64, u64)) -> usize {
    (input.0..=input.1)
        .filter(|candidate| {
            let as_str = candidate.to_string();

            let is_sorted = as_str.as_bytes().windows(2).all(|pair| {
                if let [x, y] = pair {
                    x <= y
                } else {
                    unreachable!()
                }
            });

            let has_pair = groups(as_str.bytes()).any(|group_size| group_size == 2);
            is_sorted && has_pair
        })
        .count()
}
