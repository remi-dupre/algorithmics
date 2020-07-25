// https://www.facebook.com/codingcompetitions/hacker-cup/2019/qualification-round/problems/C
use std::cmp::min;

#[derive(Clone, Copy)]
struct Cond {
    x: usize,
    nx: usize,
    c0: usize,
    c1: usize,
}

macro_rules! cond {
    ( $left:ident, $right:ident => $( $x:ident $( & $y:ident )? )|+ ) => {{
        [
            $(
                $left.$x $( + $right.$y )?,
                $right.$x $( + $left.$y )?,
            )+
        ]
        .iter()
        .min()
        .unwrap()
        .clone()
    }};
}

fn match_next(expr: &[u8]) -> (Cond, &[u8]) {
    match expr[0] {
        b'1' => (
            Cond {
                x: 1,
                nx: 1,
                c0: 1,
                c1: 0,
            },
            &expr[1..],
        ),
        b'0' => (
            Cond {
                x: 1,
                nx: 1,
                c0: 0,
                c1: 1,
            },
            &expr[1..],
        ),
        b'x' => (
            Cond {
                x: 0,
                nx: 1,
                c0: 1,
                c1: 1,
            },
            &expr[1..],
        ),
        b'X' => (
            Cond {
                x: 1,
                nx: 0,
                c0: 1,
                c1: 1,
            },
            &expr[1..],
        ),
        b'(' => {
            let (l, expr) = match_next(&expr[1..]);
            let sign = expr[0];
            let (r, tail) = match_next(&expr[1..]);

            let cond = match sign {
                b'&' => Cond {
                    c1: cond! { l, r => c1 & c1 },
                    c0: cond! { l, r => c0 | x & nx },
                    x: cond! { l, r => x & x | c1 & x },
                    nx: cond! { l, r => nx & nx | c1 & nx },
                },
                b'|' => Cond {
                    c1: cond! { l, r => c1 | x & nx },
                    c0: cond! { l, r => c0 & c0 },
                    x: cond! { l, r => x & x | c0 & x },
                    nx: cond! { l, r => nx & nx | c0 & nx },
                },
                b'^' => Cond {
                    c1: cond! { l, r => x & nx | c0 & c1 },
                    c0: cond! { l, r => x & x | nx & nx | c0 & c0 | c1 & c1 },
                    x: cond! { l, r => x & c0 | nx & c1 },
                    nx: cond! { l, r => x & c1 | nx & c0 },
                },
                _ => panic!("unexpected separator: `{}`", sign),
            };

            (cond, &tail[1..])
        }
        c => panic!("unexpected start of expression: `{}`", c),
    }
}

fn main() {
    let mut buffer = String::new();
    let cases = read_line(&mut buffer);

    for case in 1..=cases {
        let expr: String = read_line(&mut buffer);
        let (cond, _) = match_next(expr.as_bytes());
        println!("Case #{}: {}", case, min(cond.c0, cond.c1));
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
