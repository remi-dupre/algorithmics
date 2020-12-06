use std::collections::HashSet;

use fixedbitset::FixedBitSet;

pub fn generator(input: &str) -> Vec<Vec<&[u8]>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|person| person.as_bytes()).collect())
        .collect()
}

pub fn part_1_slice(groups: &[Vec<&[u8]>]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut answered = [false; 26];

            for answer in group.iter().flat_map(|person| person.iter()) {
                answered[usize::from(answer - b'a')] = true;
            }

            answered.iter().filter(|x| **x).count()
        })
        .sum()
}

pub fn part_1_bitset(groups: &[Vec<&[u8]>]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut answered = FixedBitSet::with_capacity(26);

            for answer in group.iter().flat_map(|person| person.iter()) {
                answered.set(usize::from(answer - b'a'), true);
            }

            answered.count_ones(..)
        })
        .sum()
}

pub fn part_1_hashset(groups: &[Vec<&[u8]>]) -> usize {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|person| person.iter())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn part_2(groups: &[Vec<&[u8]>]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut answered = [0usize; 26];

            for answer in group.iter().flat_map(|person| person.iter()) {
                answered[usize::from(answer - b'a')] += 1;
            }

            answered.iter().filter(|x| **x == group.len()).count()
        })
        .sum()
}
