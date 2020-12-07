use std::collections::{HashMap, HashSet};

use regex::Regex;

type Graph<'i> = HashMap<&'i str, Vec<(&'i str, usize)>>;

#[allow(clippy::clippy::match_ref_pats)]
pub fn generator(input: &str) -> Graph<'_> {
    let re = Regex::new(r"(?P<count>\d+) (?P<color>.+?) bags?").unwrap();

    let edges = input.lines().flat_map(|mut line| {
        if line.ends_with('.') {
            line = &line[..line.len() - 1];
        }

        match line.split(" bags contain ").collect::<Vec<_>>().as_slice() {
            &[source, targets] => re.captures_iter(targets).map(move |caps| {
                (
                    source,
                    caps.name("color").unwrap().as_str(),
                    caps.name("count").unwrap().as_str().parse().unwrap(),
                )
            }),
            _ => panic!("invalid constraint format"),
        }
    });

    let mut graph = HashMap::new();

    for (source, target, weight) in edges {
        graph
            .entry(source)
            .or_insert_with(Vec::new)
            .push((target, weight))
    }

    graph
}

fn reverse<'i>(graph: &Graph<'i>) -> Graph<'i> {
    let mut rev = HashMap::new();

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
    fn run<'i>(graph: &Graph<'i>, node: &'i str, seen: &mut HashSet<&'i str>) -> usize {
        if seen.contains(node) {
            0
        } else {
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
    }

    run(&reverse(graph), "shiny gold", &mut HashSet::new()) - 1
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
