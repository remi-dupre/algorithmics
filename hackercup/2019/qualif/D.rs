// https://www.facebook.com/codingcompetitions/hacker-cup/2019/qualification-round/problems/D
use std::collections::{HashMap, HashSet};

struct Uf(Vec<usize>);

impl Uf {
    fn new(n: usize) -> Self {
        Self((0..n).collect())
    }

    fn repr(&mut self, u: usize) -> usize {
        if self.0[u] == u {
            u
        } else {
            let root = self.repr(self.0[u]);
            self.0[u] = root;
            root
        }
    }

    fn merge(&mut self, u: usize, v: usize) {
        let repr_u = self.repr(u);
        let repr_v = self.repr(v);
        self.0[repr_v] = repr_u;
    }
}

fn feasible_parent(
    n: usize,
    nodes: &HashSet<usize>,
    cnst: &[(usize, usize, usize)],
    parent: usize,
) -> Option<impl Iterator<Item = HashSet<usize>>> {
    let cnst = cnst
        .iter()
        .cloned()
        .filter(|(x, y, z)| nodes.contains(x) && nodes.contains(y) && nodes.contains(z));

    // Set of pairs that must be in the same component
    let must_match = cnst
        .clone()
        .filter(|&(_, _, z)| z != parent)
        .flat_map(|(x, y, z)| vec![(x, y), (y, z)].into_iter());

    // Set of pairs that must be in different components
    let mut must_differ = cnst
        .clone()
        .filter(|&(_, _, z)| z == parent)
        .map(|(x, y, _)| (x, y));

    // Check that the root is never the child
    if cnst
        .clone()
        .any(|(x, y, z)| z != parent && (x == parent || y == parent))
    {
        return None;
    }

    // Build the components
    let mut uf = Uf::new(n);

    for (x, y) in must_match {
        uf.merge(x, y);
    }

    // Check that there is no pairs that must differ in the same component
    if must_differ.any(|(x, y)| uf.repr(x) == uf.repr(y)) {
        return None;
    }

    // Return the new components for current subtree
    let mut components: HashMap<usize, HashSet<usize>> = HashMap::new();

    for &x in nodes.iter().filter(|&&x| x != parent) {
        components.entry(uf.repr(x)).or_default().insert(x);
    }

    Some(components.into_iter().map(|(_, v)| v))
}

fn build_tree(
    n: usize,
    nodes: HashSet<usize>,
    cnst: &[(usize, usize, usize)],
    tree: &mut [usize],
) -> bool {
    let mut feasible_parents = nodes.iter().filter_map(|&parent| {
        feasible_parent(n, &nodes, cnst, parent).map(|components| (parent, components))
    });

    if let Some((parent, components)) = feasible_parents.next() {
        for component in components {
            for &x in &component {
                tree[x] = parent + 1;
            }

            if !build_tree(n, component, cnst, tree) {
                return false;
            }
        }
    } else {
        return false;
    }

    true
}

fn main() {
    let mut buffer = String::new();
    let cases = read_line(&mut buffer);

    for case in 1..=cases {
        let (n, m): (usize, usize) = read_line(&mut buffer);
        let cnst: Vec<_> = read_lines_iter(&mut buffer, m)
            .map(|(x, y, z): (usize, usize, usize)| (x - 1, y - 1, z - 1))
            .collect();

        // ---
        print!("Case #{}:", case);

        let mut tree = vec![0; n];

        if build_tree(n, (0..n).collect(), &cnst, &mut tree) {
            for x in tree {
                print!(" {}", x);
            }
            println!()
        } else {
            println!(" Impossible")
        }
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

impl<T: Read> Read for Vec<T> {
    fn read(buffer: &str) -> Self {
        buffer
            .trim()
            .split_whitespace()
            .map(|item| Read::read(item))
            .collect()
    }
}
