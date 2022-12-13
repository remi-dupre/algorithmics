use anyhow::{anyhow, bail, Result};

type Item = u8;

fn priority(item: Item) -> u64 {
    match item {
        b'a'..=b'z' => 1 + u64::from(item - b'a'),
        b'A'..=b'Z' => 27 + u64::from(item - b'A'),
        _ => panic!("invalid item: {item}"),
    }
}

pub struct RuckSack {
    compartiment_1: Vec<Item>,
    compartiment_2: Vec<Item>,
}

impl RuckSack {
    fn iter(&self) -> impl Iterator<Item = Item> + '_ {
        (self.compartiment_1.iter())
            .chain(self.compartiment_2.iter())
            .copied()
    }

    fn contains(&self, item: Item) -> bool {
        self.compartiment_1.binary_search(&item).is_ok()
            || self.compartiment_2.binary_search(&item).is_ok()
    }

    fn intersection(&self) -> impl Iterator<Item = u8> + '_ {
        (self.compartiment_1.iter())
            .copied()
            .filter(|item| self.compartiment_2.binary_search(item).is_ok())
    }
}

pub fn parse(input: &str) -> Result<Vec<RuckSack>> {
    input
        .lines()
        .map(|line| {
            let line = line.trim_end();

            if line.len() % 2 != 0 {
                bail!("got a rucksack with odd number of items");
            }

            for b in line.bytes() {
                if !b.is_ascii_alphabetic() {
                    bail!("got invalid item: {b}");
                }
            }

            let (part_1, part_2) = line.split_at(line.len() / 2);
            let mut compartiment_1: Vec<_> = part_1.bytes().collect();
            let mut compartiment_2: Vec<_> = part_2.bytes().collect();
            compartiment_1.sort_unstable();
            compartiment_2.sort_unstable();

            Ok(RuckSack {
                compartiment_1,
                compartiment_2,
            })
        })
        .collect()
}

pub fn part1(rucksack: &[RuckSack]) -> u64 {
    rucksack
        .iter()
        .flat_map(|sack| {
            let mut common: Vec<_> = sack.intersection().collect();
            common.sort_unstable();
            common.dedup();
            common
        })
        .map(priority)
        .sum()
}

pub fn part2(rucksack: &[RuckSack]) -> Result<u64> {
    rucksack
        .chunks(3)
        .map(|sacks| {
            let [sack1, sack2, sack3] = sacks else {
            bail!("number of sacks is not multiple of three");
        };

            for item in sack1.iter() {
                if sack2.contains(item) && sack3.contains(item) {
                    return Ok(priority(item));
                }
            }

            Err(anyhow!("could not find badge"))
        })
        .fold(Ok(0), |x, y| Ok(x? + y?))
}
