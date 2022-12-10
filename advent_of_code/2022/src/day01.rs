use anyhow::Result;

pub fn parse(input: &str) -> Result<Vec<Vec<u64>>> {
    let mut res = Vec::new();
    let mut curr = Vec::new();

    for line in input.lines() {
        let line = line.trim_end();

        if line.is_empty() {
            res.push(std::mem::take(&mut curr));
        } else {
            curr.push(line.parse()?);
        }
    }

    if !curr.is_empty() {
        res.push(curr);
    }

    Ok(res)
}

fn calories(input: &[Vec<u64>]) -> impl Iterator<Item = u64> + '_ {
    input.iter().map(|elf| elf.iter().sum())
}

pub fn part1(input: &[Vec<u64>]) -> u64 {
    calories(input).max().unwrap_or(0)
}

pub fn part2(input: &[Vec<u64>]) -> u64 {
    let mut calories: Vec<_> = calories(input).collect();
    calories.sort_unstable();
    calories.into_iter().rev().take(3).sum()
}
