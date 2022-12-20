use anyhow::{Context, Result};

pub fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .map(|line| line.parse().context("invalid line"))
        .collect()
}

fn decode(input: &[i64], repeat: u64) -> Vec<&i64> {
    let mut data: Vec<_> = input.iter().collect();
    let n = data.len();
    let n_i64: i64 = n.try_into().expect("input too large");

    for _ in 0..repeat {
        for x in input.iter() {
            let curr_idx = data.iter().position(|&val| std::ptr::eq(x, val)).unwrap();

            let target_idx = curr_idx
                .wrapping_add_signed(
                    (x.rem_euclid(n_i64 - 1))
                        .try_into()
                        .expect("value too large"),
                )
                .rem_euclid(n - 1);

            if curr_idx < target_idx {
                data[curr_idx..=target_idx].rotate_left(1);
            } else {
                data[target_idx..=curr_idx].rotate_right(1);
            }
        }
    }

    data
}

fn output_val(decoded: Vec<&i64>) -> i64 {
    (decoded.into_iter())
        .cycle()
        .skip_while(|x| **x != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

pub fn part1(input: &[i64]) -> i64 {
    let decoded = decode(input, 1);
    output_val(decoded)
}

pub fn part2(input: &[i64]) -> i64 {
    const KEY: i64 = 811_589_153;
    let input: Vec<_> = input.iter().map(|x| KEY * x).collect();
    let decoded = decode(&input, 10);
    output_val(decoded)
}
