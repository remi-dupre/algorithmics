use std::cmp::Ordering;

use anyhow::{bail, Context, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Packet {
    Num(u8),
    Array(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(x), Packet::Num(y)) => x.cmp(y),
            (Packet::Array(arr_x), Packet::Array(arr_y)) => {
                for (x, y) in arr_x.iter().zip(arr_y.iter()) {
                    let cmp = x.cmp(y);

                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }

                arr_x.len().cmp(&arr_y.len())
            }
            (Packet::Array(ref arr), Packet::Num(y)) => {
                let mut len_gt_one = arr.len() > 1;
                let mut arr = arr;

                loop {
                    match arr.first() {
                        Some(first) => match first {
                            Packet::Num(x) => {
                                let res = x.cmp(y);

                                if res == Ordering::Equal && len_gt_one {
                                    return Ordering::Greater;
                                }

                                break res;
                            }
                            Packet::Array(inner) => {
                                len_gt_one |= inner.len() > 1;
                                arr = inner
                            }
                        },
                        None => break Ordering::Less,
                    }
                }
            }
            (num, array) => array.cmp(num).reverse(),
        }
    }
}

fn parse_packet(buffer: &mut &[u8]) -> Result<Packet> {
    let pop_front = |buffer: &mut &[u8]| -> Option<u8> {
        let (head, tail) = buffer.split_first()?;
        *buffer = tail;
        Some(*head)
    };

    match pop_front(buffer).context("empty input")? {
        x @ b'0'..=b'9' => {
            let mut val = x - b'0';

            while let Some(k) = buffer.first() {
                if (b'0'..=b'9').contains(k) {
                    pop_front(buffer);
                    val = 10 * val + *k - b'0';
                } else {
                    break;
                }
            }

            Ok(Packet::Num(val))
        }
        b'[' => {
            let mut res = Vec::new();

            while buffer.first() != Some(&b']') {
                if buffer.first() == Some(&b',') {
                    pop_front(buffer);
                }

                res.push(parse_packet(buffer)?);
            }

            pop_front(buffer);
            Ok(Packet::Array(res))
        }
        other => bail!("invalid control char: {:?}", other as char),
    }
}

pub fn parse(input: &[u8]) -> Result<Vec<Packet>> {
    let mut buffer = input.trim_ascii_end();
    let mut res = Vec::new();

    while !buffer.is_empty() {
        res.push(parse_packet(&mut buffer)?);
        buffer = buffer.trim_ascii_start();
    }

    Ok(res)
}

pub fn part1(packets: &[Packet]) -> usize {
    packets
        .array_chunks()
        .enumerate()
        .filter(|(_, [x, y])| x <= y)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(packets: &[Packet]) -> usize {
    let make_signal = |inner| Packet::Array(vec![Packet::Array(vec![Packet::Num(inner)])]);
    let signals = [make_signal(2), make_signal(6)];
    let mut packets: Vec<_> = packets.iter().chain(&signals).collect();
    packets.sort_unstable();

    packets
        .iter()
        .enumerate()
        .filter(|(_, x)| signals.contains(x))
        .map(|(i, _)| i + 1)
        .product()
}
