use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<String>> {
    let edges = input.lines().flat_map(|line| {
        let mut splited = line.splitn(2, ')');
        let x = splited.next().unwrap();
        let y = splited.next().expect("missing parentesis");
        vec![(x, y), (y, x)]
    });

    let mut graph = HashMap::new();

    for (parent, child) in edges {
        graph
            .entry(parent.to_string())
            .or_insert_with(Vec::new)
            .push(child.to_string())
    }

    graph
}

#[aoc(day6, part1)]
pub fn part1(graph: &HashMap<String, Vec<String>>) -> u64 {
    fn count_links(
        node: &str,
        prev: Option<&str>,
        nb_parents: u64,
        graph: &HashMap<String, Vec<String>>,
    ) -> u64 {
        graph
            .get(node)
            .into_iter()
            .flatten()
            .filter(|child| Some(child.as_str()) != prev)
            .map(|child| 1 + nb_parents + count_links(child, Some(node), nb_parents + 1, graph))
            .sum()
    }

    count_links("COM", None, 0, graph)
}

#[aoc(day6, part2)]
pub fn part2(graph: &HashMap<String, Vec<String>>) -> usize {
    fn find(
        node: &str,
        target: &str,
        prev: Option<&str>,
        graph: &HashMap<String, Vec<String>>,
    ) -> Option<usize> {
        if node == target {
            Some(0)
        } else {
            graph
                .get(node)
                .into_iter()
                .flatten()
                .filter(|child| Some(child.as_str()) != prev)
                .filter_map(|child| find(child, target, Some(node), graph))
                .next()
                .map(|count| count + 1)
        }
    }

    find("YOU", "SAN", None, graph).expect("SAN is unreachable") - 2
}
