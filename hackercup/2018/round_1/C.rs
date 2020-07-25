// https://www.facebook.com/codingcompetitions/hacker-cup/2018/round-1/problems/C

use std::cmp::{Ordering, PartialOrd};

fn min(x: f64, y: f64) -> f64 {
    match x.partial_cmp(&y).unwrap() {
        Ordering::Equal | Ordering::Less => x,
        Ordering::Greater => y,
    }
}

fn max(x: f64, y: f64) -> f64 {
    match x.partial_cmp(&y).unwrap() {
        Ordering::Equal | Ordering::Less => y,
        Ordering::Greater => x,
    }
}

fn intersect(x: (f64, f64), y: (f64, f64)) -> (f64, f64) {
    (max(x.0, y.0), min(x.1, y.1))
}

fn feasible(budget: f64, height: &[f64], cnst: &[(f64, f64)]) -> bool {
    cnst.iter()
        .cloned()
        .zip(height.iter().cloned())
        .try_fold(
            (f64::NEG_INFINITY, f64::INFINITY),
            |(prev_down, prev_up), ((down, up), h)| {
                let (start, end) =
                    intersect((prev_down - down, prev_up + up), (h - budget, h + budget));

                if start > end {
                    None
                } else {
                    Some((start, end))
                }
            },
        )
        .is_some()
}

fn main() {
    let mut buffer = String::new();
    let cases = read_line(&mut buffer);

    for case in 1..=cases {
        let (n, m) = read_line(&mut buffer);
        let (h_1, h_2, w, x, y, z): (f64, f64, f64, f64, f64, f64) = read_line(&mut buffer);
        let parkourists =
            read_lines_iter(&mut buffer, m).map(|(a, b, u, d): (usize, usize, _, _)| {
                if a < b {
                    (a - 1, b - 1, u, d)
                } else {
                    (b - 1, a - 1, d, u)
                }
            });

        // Build sequence H
        let mut h = vec![h_1, h_2];

        for i in 2..n {
            h.push((w * h[i - 2] + x * h[i - 1] + y) % z);
        }

        // Compile constraint for consecutive platforms
        let mut cnst = vec![(f64::INFINITY, f64::INFINITY); n];

        for (a, b, u, d) in parkourists {
            for k in (a + 1)..=b {
                cnst[k].0 = min(cnst[k].0, d);
                cnst[k].1 = min(cnst[k].1, u);
            }
        }

        // Binary search for budget
        let mut min_budget = 0.;
        let mut max_budget =
            (h.iter().cloned().fold(0., max) - h.iter().cloned().fold(f64::INFINITY, min)) / 2.;

        while max_budget - min_budget > 1e-7 {
            let mid = (min_budget + max_budget) / 2.;

            if feasible(mid, &h, &cnst) {
                max_budget = mid;
            } else {
                min_budget = mid;
            }
        }

        println!("Case #{}: {:.6}", case, min_budget);
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
