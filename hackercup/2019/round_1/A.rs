// https://www.facebook.com/codingcompetitions/hacker-cup/2019/round-1/problems/A
use std::cmp::min;

fn floyd_warshall(graph: &mut [Vec<u64>]) {
    let n = graph.len();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if graph[i][k] < u64::MAX && graph[k][j] < u64::MAX {
                    graph[i][j] = min(graph[i][j], graph[i][k] + graph[k][j]);
                }
            }
        }
    }
}

fn main() {
    let mut buffer = String::new();

    let cases = read_line(&mut buffer);

    for case in 1..=cases {
        let (n, m) = read_line(&mut buffer);
        let constraints: Vec<_> = read_lines_iter(&mut buffer, m)
            .map(|(x, y, z): (usize, usize, u64)| (x - 1, y - 1, z))
            .collect();

        let mut graph = vec![vec![u64::MAX; n]; n];

        for &(x, y, z) in &constraints {
            graph[x][y] = z;
            graph[y][x] = z;
        }

        floyd_warshall(&mut graph);
        print!("Case #{}: ", case);

        if constraints.iter().all(|&(x, y, z)| graph[x][y] == z) {
            println!("{}", m);

            for (x, y, z) in constraints {
                println!("{} {} {}", x + 1, y + 1, z);
            }
        } else {
            println!("Impossible");
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
