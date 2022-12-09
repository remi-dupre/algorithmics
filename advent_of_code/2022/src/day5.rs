use std::cmp::Ordering;

use anyhow::{bail, Context, Result};

pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

pub struct Input {
    stacks: Vec<Vec<u8>>,
    instructions: Vec<Instruction>,
}

pub fn generator(input: &str) -> Result<Input> {
    let (input_stacks, input_insts) = input
        .split_once("\n\n")
        .context("missing separator between stacks and instructions")?;

    let mut stacks = Vec::new();

    for line in input_stacks.lines().rev().skip(1) {
        for (i, block) in (line.as_bytes())
            .chunks(4)
            .map(|block| &block[..3])
            .enumerate()
        {
            match block {
                [b' ', b' ', b' '] => {}
                [b'[', d, b']'] => {
                    let index = i;

                    let stack = loop {
                        if let Some(stack) = stacks.get_mut(index) {
                            break stack;
                        }

                        stacks.push(Vec::new());
                    };

                    stack.push(*d);
                }
                _ => bail!("invalid block format: {:?}", std::str::from_utf8(block)),
            }
        }
    }

    let instructions = input_insts
        .lines()
        .map(|line| {
            let line = line
                .strip_prefix("move ")
                .context("could not find prefix 'move' in instruction")?;

            let (count, line) = line
                .split_once(" from ")
                .context("could not find separator 'from' in instruction")?;

            let (from, to) = line
                .split_once(" to ")
                .context("could not find separator 'to' in instruction")?;

            let count = count.parse()?;
            let from = from.parse::<usize>()? - 1;
            let to = to.parse::<usize>()? - 1;
            Ok(Instruction { count, from, to })
        })
        .collect::<Result<_>>()?;

    Ok(Input {
        stacks,
        instructions,
    })
}

fn get_mut_pair<T>(slice: &mut [T], x: usize, y: usize) -> Option<(&mut T, &mut T)> {
    let (head, tail) = slice.split_at_mut(std::cmp::max(x, y));

    match x.cmp(&y) {
        Ordering::Less => Some((head.get_mut(x)?, tail.first_mut()?)),
        Ordering::Equal => None,
        Ordering::Greater => Some((tail.first_mut()?, head.get_mut(y)?)),
    }
}

fn output_encoder(stacks: Vec<Vec<u8>>) -> Result<String> {
    stacks
        .into_iter()
        .map(|stack| {
            stack
                .last()
                .map(|c| *c as char)
                .context("finished with an empty stack")
        })
        .collect()
}

pub fn part1(input: &Input) -> Result<String> {
    let mut stacks = input.stacks.clone();

    for inst in &input.instructions {
        let (from, to) = get_mut_pair(&mut stacks, inst.from, inst.to)
            .context("could not get pair (from, to)")?;

        let moved = from.drain(from.len() - inst.count..).rev();
        to.extend(moved);
    }

    output_encoder(stacks)
}

pub fn part2(input: &Input) -> Result<String> {
    let mut stacks = input.stacks.clone();

    for inst in &input.instructions {
        let (from, to) = get_mut_pair(&mut stacks, inst.from, inst.to)
            .context("could not get pair (from, to)")?;

        let moved = from.drain(from.len() - inst.count..);
        to.extend(moved);
    }

    output_encoder(stacks)
}
