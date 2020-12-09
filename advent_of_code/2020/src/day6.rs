use std::collections::HashSet;

use fixedbitset::FixedBitSet;

pub fn generator(input: &str) -> Vec<Vec<&[u8]>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|person| person.as_bytes()).collect())
        .collect()
}

pub fn part_1_u32(groups: &[Vec<&[u8]>]) -> u32 {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|person| person.iter().map(|&answer| 1u32 << (answer - b'a')))
                .fold(0, |acc, per| acc | per)
                .count_ones()
        })
        .sum()
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

pub fn part_2_u32(groups: &[Vec<&[u8]>]) -> u32 {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|person| {
                    person
                        .iter()
                        .map(|&answer| 1 << (answer - b'a'))
                        .fold(0, |acc, per| acc | per)
                })
                .fold(u32::MAX, |acc, grp| acc & grp)
                .count_ones()
        })
        .sum()
}

pub fn part_2_slice(groups: &[Vec<&[u8]>]) -> usize {
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

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day6::*;

    const EXAMPLE: &str = crate::lines! {
        "abc"
        ""
        "a"
        "b"
        "c"
        ""
        "ab"
        "ac"
        ""
        "a"
        "a"
        "a"
        "a"
        ""
        "b"
    };

    #[test]
    fn test_part_1() {
        let groups = generator(EXAMPLE);
        assert_eq!(11, part_1_u32(&groups));
        assert_eq!(11, part_1_slice(&groups));
        assert_eq!(11, part_1_bitset(&groups));
        assert_eq!(11, part_1_hashset(&groups));
    }

    #[test]
    fn test_part_2() {
        let groups = generator(EXAMPLE);
        assert_eq!(6, part_2_u32(&groups));
        assert_eq!(6, part_2_slice(&groups));
    }
}
