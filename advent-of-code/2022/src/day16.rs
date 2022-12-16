use anyhow::{Context, Result};
use fxhash::{FxHashMap, FxHashSet};

use crate::util::matrix::Matrix;

type FlowRate = u32;

#[derive(Debug)]
pub struct Valve<'i> {
    name: &'i str,
    flow: FlowRate,
    tunnels: Vec<&'i str>,
}

impl Valve<'_> {
    fn is_relevant(&self) -> bool {
        self.flow > 0 || self.name == "AA"
    }
}

fn build_subgraph(valves: &[Valve]) -> Matrix<usize> {
    let name_to_id: FxHashMap<_, _> = (valves.iter())
        .enumerate()
        .map(|(i, valve)| (valve.name, i))
        .collect();

    // Run a Floyd-Warshall algorithm to compute distance between all pairs of nodes
    let n = valves.len();
    let mut dist = Matrix::new(n, n, usize::MAX);

    for k in 0..n {
        dist[(k, k)] = 0;
    }

    for (i, valve) in valves.iter().enumerate() {
        for edge in &valve.tunnels {
            let j = name_to_id[edge];
            dist[(i, j)] = 1;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[(i, j)] =
                    std::cmp::min(dist[(i, j)], dist[(i, k)].saturating_add(dist[(k, j)]));
            }
        }
    }

    // Extract only the relevant nodes from the graph (i.e. nodes with a flow > 0, or starting
    // node)
    let relevant_nodes: FxHashSet<_> = valves
        .iter()
        .enumerate()
        .filter(|(_, valve)| valve.is_relevant())
        .map(|(i, _)| i)
        .collect();

    Matrix::try_from_iter(
        relevant_nodes.len(),
        relevant_nodes.len(),
        dist.iter()
            .filter(|((i, j), _)| relevant_nodes.contains(i) && relevant_nodes.contains(j))
            .map(|(_, weight)| Ok(*weight)),
    )
    .expect("invalid output subgraph size")
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
pub fn parse(input: &str) -> Result<Vec<Valve>> {
    input
        .lines()
        .map(|line| {
            let line = line
                .strip_prefix("Valve ")
                .context("missing valve prefix")?;

            let (name, line) = line
                .split_once(" has flow rate=")
                .context("missing flow rate")?;

            let (flow, line) = (line.split_once("; tunnels lead to valves "))
                .or_else(|| line.split_once("; tunnel leads to valve "))
                .context("missing valves")?;

            let tunnels = line.split(", ").collect();
            let flow = flow.parse().context("invalid flow rate")?;

            let valve = Valve {
                name,
                flow,
                tunnels,
            };

            Ok(valve)
        })
        .collect()
}

fn has_bit(bitset: u16, bit: usize) -> bool {
    (bitset.to_le() >> bit) & 1 != 0
}

fn set_bit(bitset: u16, bit: usize) -> u16 {
    u16::from_le(bitset.to_le() | (1 << bit))
}

const TIME_STEPS: usize = 29;

pub fn part1(valves: &[Valve<'_>]) -> u32 {
    let dist = build_subgraph(valves);
    let valves: Vec<_> = valves.iter().filter(|valve| valve.is_relevant()).collect();

    let start_idx = valves
        .iter()
        .enumerate()
        .find(|(_, valve)| valve.name == "AA")
        .expect("missing starting valve")
        .0;

    let n = dist.width();

    assert!(n <= u16::BITS as _);
    let bitset_max_val = 2u16.wrapping_pow(n as _).wrapping_sub(1);

    let col = |pos: usize, bitset: u16| usize::from(bitset) * n + pos;
    let mut matrix = Matrix::new((usize::from(bitset_max_val) + 1) * n + n, TIME_STEPS + 1, 0);
    matrix[(col(start_idx, 0), 0)] = 1;

    for t in 0..TIME_STEPS {
        for bitset in 0..=bitset_max_val {
            for from in 0..n {
                // Skip if we can't reach this pos
                if matrix[(col(from, bitset), t)] == 0 {
                    continue;
                }

                // // Open current valve
                // if !has_bit(bitset, from) {
                //     let next_bitset = set_bit(bitset, from);
                //
                //     matrix[(col(from, next_bitset), t + 1)] = std::cmp::max(
                //         matrix[(col(from, next_bitset), t + 1)],
                //         matrix[(col(from, bitset), t)]
                //             + ((TIME_STEPS - t) as u32) * valves[from].flow,
                //     );
                // }

                // Jump to neighbours and swap valve
                for to in 0..n {
                    if has_bit(bitset, to) {
                        continue;
                    }

                    let dist = dist[(from, to)];

                    if t + dist + 1 > TIME_STEPS {
                        continue;
                    }

                    let next_bitset = set_bit(bitset, to);

                    matrix[(col(to, next_bitset), t + dist + 1)] = std::cmp::max(
                        matrix[(col(to, next_bitset), t + dist + 1)],
                        matrix[(col(from, bitset), t)]
                            + ((TIME_STEPS - t - dist) as u32) * valves[to].flow,
                    );
                }
            }
        }
    }

    eprintln!("matrix size: {}", matrix.width() * matrix.height());

    eprintln!(
        "set size: {}",
        matrix.iter().filter(|(_, x)| **x != 0).count()
    );

    matrix.iter().map(|(_, x)| *x).max().unwrap() - 1
}
