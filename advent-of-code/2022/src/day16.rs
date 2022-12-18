use std::cell::RefCell;

use anyhow::{Context, Result};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::util::matrix::Matrix;

type FlowRate = u32;

pub struct Valve<'i> {
    name: &'i str,
    flow: FlowRate,
    tunnels: Vec<&'i str>,
}

pub struct Input<'i> {
    valves: Vec<Valve<'i>>,
    precomputed: RefCell<[Vec<u32>; 2]>,
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
pub fn parse(input: &str) -> Result<Input> {
    let mut valves: Vec<_> = input
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
        .collect::<Result<_>>()?;

    let start_idx = valves
        .iter()
        .enumerate()
        .find(|(_, valve)| valve.name == "AA")
        .context("missing valve with name 'AA'")?
        .0;

    let last_idx = valves.len() - 1;
    valves.swap(start_idx, last_idx);

    Ok(Input {
        valves,
        precomputed: Default::default(),
    })
}

fn has_bit(bitset: u16, bit: u8) -> bool {
    (bitset.to_le() >> bit) & 1 != 0
}

fn set_bit(bitset: u16, bit: u8) -> u16 {
    u16::from_le(bitset.to_le() | (1 << bit))
}

fn solve<const RES_SIZE: usize>(
    valves: &[Valve],
    time_steps_in_res: [usize; RES_SIZE],
) -> [Vec<u32>; RES_SIZE] {
    let max_steps = time_steps_in_res.into_iter().max().unwrap_or(0);

    // Build a subgraph of relevant nodes
    let dist = build_subgraph(valves);
    let valves: Vec<_> = valves.iter().filter(|valve| valve.is_relevant()).collect();

    // State should fit in an u16
    let n: u8 = valves.len().try_into().expect("too many relevant valves");
    assert!(u32::from(n) <= u16::BITS, "too many relevant valves");

    // The starting node should be in last position. We can also assume that it has no flow which
    // will reduce the number of states to consider.
    assert_eq!(valves[usize::from(n - 1)].name, "AA");
    assert_eq!(valves[usize::from(n - 1)].flow, 0);

    // Keep track of the best score that can be obtained, at a given time, given position and with
    // some state of the valves (represented by a bitset)
    let mut best_at = vec![FxHashMap::default(); max_steps + 1];
    best_at[0].insert((n - 1, 0), 0);

    // Compute the best value for any given configuration of opened valves
    let mut res = std::array::from_fn(|_| vec![0; 2usize.pow((n - 1).into())]);

    // As there is no reason to move from a position to another without immediately opening a
    // valve, thus we can implement only "move + open" steps. This little trick will allow to
    // eliminate a lot of states that we would have to consider otherwise.
    for t in 0..max_steps {
        std::mem::take(&mut best_at[t]).into_iter().for_each(
            |((from, opened_valves), from_total)| {
                for to in 0..(n - 1) {
                    if has_bit(opened_valves, to) {
                        continue;
                    }

                    let dist = dist[(from.into(), to.into())];
                    let new_t = t + dist + 1;

                    let Some(to_time_state) = best_at.get_mut(new_t) else {
                        continue;
                    };

                    let valve = valves[usize::from(to)];
                    let new_opened_valves = set_bit(opened_valves, to);
                    let new_total = from_total + ((max_steps - t - dist) as u32) * valve.flow;

                    to_time_state
                        .entry((to, new_opened_valves))
                        .and_modify(|val| *val = std::cmp::max(*val, new_total))
                        .or_insert(new_total);

                    for (res_steps, res) in time_steps_in_res.into_iter().zip(&mut res) {
                        if new_t <= res_steps {
                            let idx = usize::from(new_opened_valves);

                            res[idx] = std::cmp::max(res[idx], new_total);
                        }
                    }
                }
            },
        )
    }

    // Remove extra value for early results
    for (res_steps, res) in time_steps_in_res.into_iter().zip(&mut res) {
        if res_steps == max_steps {
            continue;
        }

        for (layout, val) in res.iter_mut().enumerate() {
            if *val == 0 {
                continue;
            }

            let layout: u16 = layout as u16;

            for idx in 0..(n - 1) {
                if has_bit(layout, idx) {
                    *val -= (max_steps - res_steps) as u32 * valves[usize::from(idx)].flow;
                }
            }
        }
    }

    res
}

pub fn precomputing(input: &Input) -> &'static str {
    let value = solve(&input.valves, [29, 25]);
    *input.precomputed.borrow_mut() = value;
    "scores for [29, 25] steps"
}

pub fn part1(input: &Input) -> u32 {
    let precomputed = input.precomputed.borrow();
    *precomputed[0].iter().max().unwrap_or(&0)
}

pub fn part2(input: &Input) -> u32 {
    // Retrieve precomputed data
    let precomputed = input.precomputed.borrow();
    let score_for_layout = &precomputed[1];

    // Find size of the output (number of bits of state and mask for layouts)
    let bound: u16 = score_for_layout
        .len()
        .try_into()
        .expect("must fit in 16 bits");

    let bit_bound =
        u8::try_from(score_for_layout.len().trailing_zeros()).expect("must fit in 16 bits");

    let bit_mask = u16::MAX >> (16 - bit_bound);

    // Use DP to compute best complement value for any layouts using best value for layouts with
    // one less valve openened
    let layouts_per_count_zeroes = (0..bound)
        .into_par_iter()
        .fold_with(Default::default(), |mut acc: [Vec<u16>; 17], layout| {
            let idx: usize = layout.count_zeros() as _;
            acc[idx].push(layout);
            acc
        })
        .reduce(Default::default, |mut acc, other| {
            for (from_acc, from_other) in acc.iter_mut().zip(other) {
                from_acc.extend_from_slice(&from_other);
            }

            acc
        });

    let mut comp_for_layout = vec![0; score_for_layout.len()];

    for layouts in layouts_per_count_zeroes.into_iter() {
        let todo: Vec<_> = layouts
            .into_par_iter()
            .map(|layout| {
                let from_smaller = (0..bit_bound)
                    .filter(|&bit| !has_bit(layout, bit))
                    .map(|bit| comp_for_layout[usize::from(set_bit(layout, bit))])
                    .max()
                    .unwrap_or(0);

                (
                    layout,
                    std::cmp::max(
                        from_smaller,
                        score_for_layout[usize::from(bit_mask & !layout)],
                    ),
                )
            })
            .collect();

        for (layout, val) in todo {
            comp_for_layout[usize::from(layout)] = val;
        }
    }

    score_for_layout
        .iter()
        .zip(comp_for_layout)
        .map(|(val_1, val_2)| *val_1 + val_2)
        .max()
        .unwrap_or(0)
}
