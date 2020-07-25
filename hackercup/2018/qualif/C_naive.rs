// https://www.facebook.com/codingcompetitions/hacker-cup/2018/qualification-round/problems/C
// The naive version runs in O(n²) which is accepted with given outputs.

fn ethan(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();

    let mut i = 0;
    let mut j = 0;

    loop {
        if i >= a.len() {
            break true;
        }

        if j >= b.len() {
            break false;
        }

        if a[i] == b[j] {
            i += 1;
            j += 1;
        } else if i == 0 {
            j += 1
        } else {
            i = 0;
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let cases = read_line(&mut buffer);

    'cases: for case in 1..=cases {
        let text: String = read_line(&mut buffer);

        for k in 1..=text.len() {
            let attempt = text[0..k].to_string() + &text;

            if !ethan(&text, &attempt) {
                println!("Case #{}: {}", case, attempt);
                continue 'cases;
            }
        }

        println!("Case #{}: Impossible", case);
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
