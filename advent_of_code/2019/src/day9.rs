use aoc_runner_derive::{aoc, aoc_generator};

use crate::computer::Computer;
use crate::day2;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<isize> {
    day2::input_generator(input)
}

#[aoc(day9, part1)]
pub fn part1(program: &[isize]) -> isize {
    let mut input = Some(1);

    Computer::new(program, || {
        input.take().expect("program reads inputs twice")
    })
    .next()
    .expect("computer didn't output")
}

#[aoc(day9, part2)]
pub fn part2(program: &[isize]) -> isize {
    let mut input = Some(2);

    Computer::new(program, || {
        input.take().expect("program reads inputs twice")
    })
    .next()
    .expect("computer didn't output")
}
