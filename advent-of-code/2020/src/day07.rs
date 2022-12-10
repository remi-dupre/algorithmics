use std::error::Error;

use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

type Graph<'i> = FxHashMap<&'i str, Vec<(&'i str, usize)>>;

pub fn generator(input: &str) -> Result<Graph, Box<dyn Error>> {
    let re = Regex::new(r"(?P<count>\d+) (?P<color>.+?) bags?").unwrap();

    let edges = input.lines().flat_map(|mut line| {
        if line.ends_with('.') {
            line = &line[..line.len() - 1];
        }

        match *line.split(" bags contain ").collect::<Vec<_>>().as_slice() {
            [source, targets] => re.captures_iter(targets).map(move |caps| {
                Ok::<_, Box<dyn Error>>((
                    source,
                    caps.name("color").ok_or("missing color")?.as_str(),
                    caps.name("count")
                        .ok_or("missing count")?
                        .as_str()
                        .parse()?,
                ))
            }),
            _ => panic!("invalid constraint format"),
        }
    });

    let mut graph = FxHashMap::default();

    for edge in edges {
        let (source, target, weight) = edge?;
        graph
            .entry(source)
            .or_insert_with(Vec::new)
            .push((target, weight))
    }

    Ok(graph)
}

fn reverse<'i>(graph: &Graph<'i>) -> Graph<'i> {
    let mut rev = FxHashMap::default();

    for (source, target, weight) in graph.iter().flat_map(|(&source, targets)| {
        targets
            .iter()
            .copied()
            .map(move |(target, weight)| (source, target, weight))
    }) {
        rev.entry(target)
            .or_insert_with(Vec::new)
            .push((source, weight))
    }

    rev
}

pub fn part_1(graph: &Graph) -> usize {
    fn run<'i>(graph: &Graph<'i>, node: &'i str, seen: &mut FxHashSet<&'i str>) -> usize {
        if seen.contains(node) {
            return 0;
        }

        seen.insert(node);

        if let Some(children) = graph.get(node) {
            1 + children
                .iter()
                .map(|&(child, _)| run(graph, child, seen))
                .sum::<usize>()
        } else {
            1
        }
    }

    run(&reverse(graph), "shiny gold", &mut FxHashSet::default()) - 1
}

pub fn part_2(graph: &Graph) -> usize {
    fn run<'i>(graph: &Graph<'i>, node: &'i str) -> usize {
        if let Some(children) = graph.get(node) {
            1 + children
                .iter()
                .map(|&(child, count)| count * run(graph, child))
                .sum::<usize>()
        } else {
            1
        }
    }

    run(&graph, "shiny gold") - 1
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day07::*;

    const EXAMPLE_1: &str = crate::lines! {
        "light red bags contain 1 bright white bag, 2 muted yellow bags."
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags."
        "bright white bags contain 1 shiny gold bag."
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags."
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."
        "faded blue bags contain no other bags."
        "dotted black bags contain no other bags."
    };

    const EXAMPLE_2: &str = crate::lines! {
        "shiny gold bags contain 2 dark red bags."
        "dark red bags contain 2 dark orange bags."
        "dark orange bags contain 2 dark yellow bags."
        "dark yellow bags contain 2 dark green bags."
        "dark green bags contain 2 dark blue bags."
        "dark blue bags contain 2 dark violet bags."
        "dark violet bags contain no other bags."
    };

    #[test]
    fn test_part_1() {
        assert_eq!(4, part_1(&generator(EXAMPLE_1).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(126, part_2(&generator(EXAMPLE_2).unwrap()));
    }
}
