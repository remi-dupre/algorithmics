use aoc_runner_derive::{aoc, aoc_generator};

use crate::computer::Computer;
use crate::day2;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<isize> {
    day2::input_generator(input)
}

#[aoc(day5, part1)]
pub fn part1(program: &[isize]) -> isize {
    let mut input = Some(1);

    Computer::new(program, || {
        input.take().expect("program reads inputs twice")
    })
    .last()
    .expect("computer didn't output")
}

#[aoc(day5, part2)]
pub fn part2(program: &[isize]) -> isize {
    let mut input = Some(5);

    Computer::new(program, || {
        input.take().expect("program reads inputs twice")
    })
    .last()
    .expect("computer didn't output")
}
