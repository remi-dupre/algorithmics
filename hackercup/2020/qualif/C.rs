// https://www.facebook.com/codingcompetitions/hacker-cup/2020/qualification-round/problems/C

use std::cmp::max;
use std::collections::{HashMap, HashSet};

type Node = (i64, bool);

fn furthest(graph: &HashMap<Node, Vec<Node>>) -> i64 {
    let mut seen = HashSet::new();
    let mut best = 0;

    let mut nodes: Vec<_> = graph.keys().cloned().collect();
    nodes.sort_unstable();

    for start in nodes {
        if seen.contains(&start) {
            continue;
        }

        let mut run = vec![start];
        seen.insert(start);

        while let Some(curr) = run.pop() {
            best = max(best, curr.0 - start.0);

            for &neighbour in graph.get(&curr).map(Vec::as_slice).unwrap_or(&[]) {
                if !seen.contains(&neighbour) {
                    seen.insert(neighbour);
                    run.push(neighbour);
                }
            }
        }
    }

    best
}

fn main() {
    let mut buffer = String::new();
    let cases = read_line(&mut buffer);

    for case in 1..=cases {
        let n = read_line(&mut buffer);
        let edges = read_lines_iter(&mut buffer, n).flat_map(|(p, h): (i64, i64)| {
            vec![
                ((p - h, true), (p, false)),
                ((p - h, false), (p, false)),
                ((p, true), (p + h, false)),
                ((p, true), (p + h, true)),
            ]
            .into_iter()
        });

        let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();

        for (u, v) in edges {
            graph.entry(u).or_default().push(v)
        }

        println!("Case #{}: {}", case, furthest(&graph));
    }
}

/// ---
/// --- Read primitives
/// ---

pub fn read_line<T: Read>(buffer: &mut String) -> T {
    buffer.clear();

    std::io::stdin()
        .read_line(buffer)
        .expect("failed to read stdin");

    Read::read(buffer)
}

pub fn read_line_iter<T: Read>(buffer: &mut String) -> impl Iterator<Item = T> + '_ {
    buffer.clear();

    std::io::stdin()
        .read_line(buffer)
        .expect("failed to read stdin");

    buffer
        .trim()
        .split_whitespace()
        .map(|item| Read::read(item))
}

pub fn read_lines_iter<T: Read>(
    buffer: &mut String,
    nb_lines: usize,
) -> impl Iterator<Item = T> + '_ {
    (0..nb_lines).map(move |_| read_line(buffer))
}

pub trait Read {
    fn read(buffer: &str) -> Self;
}

macro_rules! impl_basic {
    ( $type:ty ) => {
        impl Read for $type {
            fn read(buffer: &str) -> Self {
                buffer
                    .trim()
                    .parse()
                    .map_err(|err| format!("failed to parse {:?}: {:?}", buffer, err))
                    .unwrap()
            }
        }
    };
}

impl_basic!(u8);
impl_basic!(i8);
impl_basic!(u32);
impl_basic!(i32);
impl_basic!(u64);
impl_basic!(i64);
impl_basic!(usize);
impl_basic!(isize);
impl_basic!(f32);
impl_basic!(f64);
impl_basic!(bool);
impl_basic!(char);
impl_basic!(String);

macro_rules! impl_tuple {
    ( $($name:ident),+ ) => {
        #[allow(non_snake_case)]
        impl<$($name: Read),+> Read for ($($name),+) {
            fn read(buffer: &str) -> Self {
                let mut iter = buffer.split_whitespace().map(|line| line.trim());

                $(
                    let $name = Read::read(iter.next().expect("not enough items for tuple"));
                )+

                if iter.next().is_some() {
                    panic!("too many items for tuple");
                }

                ($($name),+)
            }
        }
    }
}

impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);

impl Read for () {
    fn read(_buffer: &str) -> Self {}
}

impl<T: Read> Read for Vec<T> {
    fn read(buffer: &str) -> Self {
        buffer
            .trim()
            .split_whitespace()
            .map(|item| Read::read(item))
            .collect()
    }
}

// ---
// --- Sequence: handy for pseudo-random sequences given as input.
// ---

pub struct Sequence<T: Copy, S: Fn(T) -> T> {
    state: T,
    step: S,
}

impl<T: Copy, S: Fn(T) -> T> Sequence<T, S> {
    pub fn new(init: T, step: S) -> Self {
        Self { state: init, step }
    }
}

impl<T: Copy, S: Fn(T) -> T> Iterator for Sequence<T, S> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current = self.state;
        self.state = (self.step)(current);
        Some(current)
    }
}
