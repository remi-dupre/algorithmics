use std::convert::TryInto;
use std::error::Error;

use rustc_hash::FxHashMap;

pub enum Op {
    Set { addr: u64, val: u64 },
    BitMask(Box<[u8; 36]>),
}

pub fn generator(input: &str) -> Result<Vec<Op>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            if let Some(mask) = line.strip_prefix("mask = ") {
                Ok(Op::BitMask(Box::new(mask.as_bytes().try_into()?)))
            } else if let Some(assign) = line.strip_prefix("mem[") {
                let (addr, val) = assign
                    .split_once("] = ")
                    .ok_or("invalid assignation format")?;

                Ok(Op::Set {
                    addr: addr.parse()?,
                    val: val.parse()?,
                })
            } else {
                Err("unknown operation".into())
            }
        })
        .collect()
}

pub fn part_1(operations: &[Op]) -> u64 {
    fn apply_mask(mask: &[u8; 36], input_val: u64) -> u64 {
        mask.iter()
            .rev()
            .fold((1, 0), |(idx, val), b| match b {
                b'X' => (idx << 1, val | (idx & input_val)),
                b'1' => (idx << 1, val | idx),
                b'0' => (idx << 1, val),
                _ => panic!("invalid mask"),
            })
            .1
    }

    let mem = operations
        .iter()
        .fold(
            (FxHashMap::default(), &[0; 36]),
            |(mut mem, mask), op| match op {
                Op::Set { addr, val } => {
                    mem.insert(*addr, apply_mask(&mask, *val));
                    (mem, mask)
                }
                Op::BitMask(mask) => (mem, mask),
            },
        )
        .0;

    mem.into_values().sum()
}

pub fn part_2(operations: &[Op]) -> u64 {
    fn apply_mask(mask: &[u8], addr: u64) -> Vec<u64> {
        match mask {
            [] => vec![1],
            [head @ .., m] => {
                let vals = apply_mask(head, addr >> 1).into_iter();
                match m {
                    b'1' => vals.map(|x| (x << 1) + 1).collect(),
                    b'0' => vals.map(|x| (x << 1) + (addr & 1)).collect(),
                    b'X' => vals.flat_map(|x| vec![(x << 1), (x << 1) + 1]).collect(),
                    _ => panic!("invalid bitmap"),
                }
            }
        }
    }

    let mem = operations
        .iter()
        .fold(
            (FxHashMap::default(), &[0; 36]),
            |(mut mem, mask), op| match op {
                Op::Set { addr, val } => {
                    for addr in apply_mask(mask, *addr) {
                        mem.insert(addr, *val);
                    }

                    (mem, mask)
                }
                Op::BitMask(mask) => (mem, mask),
            },
        )
        .0;

    mem.into_values().sum()
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day14::*;

    const EXAMPLE_1: &str = crate::lines! {
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        "mem[8] = 11"
        "mem[7] = 101"
        "mem[8] = 0"
    };

    const EXAMPLE_2: &str = crate::lines! {
        "mask = 000000000000000000000000000000X1001X"
        "mem[42] = 100"
        "mask = 00000000000000000000000000000000X0XX"
        "mem[26] = 1"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(165, part_1(&generator(EXAMPLE_1).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(208, part_2(&generator(EXAMPLE_2).unwrap()));
    }
}
