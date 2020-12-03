use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .unwrap_or_else(|err| panic!("invalid number `{}`: `{}`", line, err))
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u64]) -> u64 {
    input.iter().map(|&mass| mass / 3 - 2).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u64]) -> u64 {
    fn total_needed_mass(obj: u64) -> u64 {
        if obj < 9 {
            0
        } else {
            let obj_mass = obj / 3 - 2;
            obj_mass + total_needed_mass(obj_mass)
        }
    }

    input.iter().copied().map(total_needed_mass).sum()
}
