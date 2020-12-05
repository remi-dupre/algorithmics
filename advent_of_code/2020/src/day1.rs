use std::collections::HashSet;

const TARGET: u32 = 2020;

pub fn generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .unwrap_or_else(|err| panic!("invalid number `{}`: `{}`", line, err))
        })
        .collect()
}

pub fn part_1_array(input: &[u32]) -> u32 {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();

    for &num in &numbers {
        if numbers.binary_search(&(TARGET - num)).is_ok() {
            return num * (TARGET - num);
        }
    }

    unreachable!("no feasible solution")
}

pub fn part_1_hashset(input: &[u32]) -> u32 {
    let numbers: HashSet<_> = input.iter().copied().collect();

    for &num in input {
        if numbers.contains(&(TARGET - num)) {
            return num * (TARGET - num);
        }
    }

    unreachable!("no feasible solution")
}

pub fn part_2(input: &[u32]) -> u32 {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();

    for &x in &numbers {
        for &y in &numbers {
            if x + y <= TARGET && numbers.binary_search(&(TARGET - x - y)).is_ok() {
                return x * y * (TARGET - x - y);
            }
        }
    }

    unreachable!("no feasible solution")
}
