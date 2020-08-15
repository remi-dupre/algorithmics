// https://www.facebook.com/codingcompetitions/hacker-cup/2018/round-2/problems/B

fn subtree(root: usize, tree: &[Vec<usize>]) -> Vec<usize> {
    let mut subtree = vec![root];
    let mut cursor = 0;

    while let Some(&u) = subtree.get(cursor) {
        cursor += 1;

        for &v in &tree[u] {
            subtree.push(v);
        }
    }

    subtree
}

fn max_weight_from(start: usize, skip: bool, visited: &mut [bool], graph: &[Vec<usize>]) -> usize {
    let mut total = 0;
    visited[start] = true;

    if !skip {
        eprintln!("{}", start);
        total += start;
    }

    for &next in &graph[start] {
        if !visited[next] {
            total += max_weight_from(next, !skip, visited, graph);
        }
    }

    total
}

fn max_weight(graph: &[Vec<usize>]) -> usize {
    max_weight_from(0, false, &mut vec![false; graph.len()], graph)
}

fn main() {
    let mut buffer = String::new();
    let cases = read_line(&mut buffer);

    for case in 1..=cases {
        let (n, m, a, b): (usize, usize, usize, usize) = read_line(&mut buffer);

        let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];

        for (i, parent) in read_lines_iter::<usize>(&mut buffer, n - 1).enumerate() {
            tree[parent].push(i + 1);
        }

        let queries = Sequence::new((0, 0), |(i, _)| (i + 1, (a * i + b) % n))
            .map(|(_i, x)| x)
            .skip(1)
            .take(m);

        // The n first elements represents candies and m last represent customers.
        let mut graph = vec![Vec::new(); n + m];

        for (customer, root) in queries.enumerate() {
            for query in subtree(root, &tree) {
                graph[query].push(n + customer);
                graph[n + customer].push(query);
            }
        }

        println!("Case #{}: {}", case, max_weight(&graph));
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
