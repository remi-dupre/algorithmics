use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::RangeInclusive;

pub struct Input<'i> {
    rules: Vec<(&'i str, RangeInclusive<u16>)>,
    nearby_tickets: Vec<Vec<u16>>,
    ticket: Vec<u16>,
}

pub fn generator(input: &str) -> Result<Input<'_>, Box<dyn Error>> {
    let mut blocks = input.split("\n\n");

    let rules = {
        let rule_block = blocks.next().ok_or("missing rule block")?;
        rule_block
            .lines()
            .flat_map(
                |line| match *line.split(": ").collect::<Vec<_>>().as_slice() {
                    [key, ranges] => ranges.split(" or ").map(move |range| {
                        match range.split('-').collect::<Vec<_>>().as_slice() {
                            [left, right] => Ok((key, left.parse::<u16>()?..=right.parse()?)),
                            _ => Err("invalid range format".into()),
                        }
                    }),
                    _ => panic!(),
                },
            )
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?
    };

    let ticket = {
        blocks
            .next()
            .ok_or("missing ticket block")?
            .lines()
            .nth(1)
            .ok_or("empty ticket block")?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?
    };

    let nearby_tickets = {
        blocks
            .next()
            .ok_or("missing nearby_tickets block")?
            .lines()
            .skip(1)
            .map(|line| line.split(',').map(str::parse).collect())
            .collect::<Result<_, _>>()?
    };

    Ok(Input {
        rules,
        nearby_tickets,
        ticket,
    })
}

pub fn part_1(input: &Input) -> u16 {
    let valid_domain: HashSet<_> = input
        .rules
        .iter()
        .flat_map(|(_, rng)| rng.clone())
        .collect();

    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&x| !valid_domain.contains(x))
        .sum()
}

// Pop a value from a small vec
fn pop_vec<T: Eq>(vals: &mut Vec<T>, val: &T) -> Option<T> {
    let (idx, _) = vals.iter().enumerate().find(|&(_, x)| x == val)?;
    Some(vals.swap_remove(idx))
}

pub fn part_2(input: &Input) -> Option<u128> {
    // Set of all fields
    let mut all_fields: Vec<_> = input.rules.iter().map(|&(field, _)| field).collect();
    all_fields.sort_unstable();
    all_fields.dedup();

    // For a given value: the set of feasible fields it can hold
    let feasible_for_val = {
        let mut res = HashMap::new();

        for (field, rng) in &input.rules {
            for val in rng.clone() {
                res.entry(val).or_insert_with(Vec::new).push(*field);
            }
        }

        for (_, fields) in res.iter_mut() {
            fields.sort_unstable();
            fields.dedup();
        }

        res
    };

    let fields_mapping: HashMap<_, _> = {
        let mut feasible_fields = HashMap::new();

        for ticket in &input.nearby_tickets {
            if !ticket.iter().all(|val| feasible_for_val.contains_key(val)) {
                continue;
            }

            for (i, &x) in ticket.iter().enumerate() {
                if let Some(feasible_for_x) = feasible_for_val.get(&x) {
                    feasible_fields
                        .entry(i)
                        .and_modify(|curr_feasibles: &mut Vec<_>| {
                            curr_feasibles.retain(|field| feasible_for_x.contains(field))
                        })
                        .or_insert_with(|| feasible_for_x.clone());
                }
            }
        }

        std::iter::from_fn({
            move || {
                let (idx, removed) = feasible_fields
                    .iter()
                    .find(|(_, feasibles)| feasibles.len() == 1)
                    .map(|(idx, feasibles)| (*idx, *feasibles.iter().next().unwrap()))?;

                for (_, feasibles) in feasible_fields.iter_mut() {
                    pop_vec(feasibles, &removed);
                }

                Some((removed, idx))
            }
        })
        .collect()
    };

    all_fields
        .into_iter()
        .filter(|field| field.starts_with("departure"))
        .map(|field| input.ticket.get(*fields_mapping.get(field)?))
        .try_fold(1, |acc, x| Some(acc * u128::from(*x?)))
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day16::*;

    const EXAMPLE_1: &str = crate::lines! {
        "class: 1-3 or 5-7"
        "row: 6-11 or 33-44"
        "seat: 13-40 or 45-50"
        ""
        "your ticket:"
        "7,1,14"
        ""
        "nearby tickets:"
        "7,3,47"
        "40,4,50"
        "55,2,20"
        "38,6,12"
    };

    const EXAMPLE_2: &str = crate::lines! {
        "departure class: 0-1 or 4-19"
        "departure row: 0-5 or 8-19"
        "departure seat: 0-13 or 16-19"
        ""
        "your ticket:"
        "11,12,13"
        ""
        "nearby tickets:"
        "3,9,18"
        "15,1,5"
        "5,14,9"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(71, part_1(&generator(EXAMPLE_1).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(11 * 12 * 13, part_2(&generator(EXAMPLE_2).unwrap()));
    }
}
