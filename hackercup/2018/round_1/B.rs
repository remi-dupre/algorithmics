fn run_order_traversals(
    from: usize,
    pre: &mut Vec<usize>,
    post: &mut Vec<usize>,
    tree: &[[Option<usize>; 2]],
) {
    pre.push(from);

    if let Some(l) = tree[from][0] {
        run_order_traversals(l, pre, post, tree)
    }

    if let Some(r) = tree[from][1] {
        run_order_traversals(r, pre, post, tree)
    }

    post.push(from);
}

fn order_traversals(tree: &[[Option<usize>; 2]]) -> (Vec<usize>, Vec<usize>) {
    let mut pre = Vec::new();
    let mut post = Vec::new();
    run_order_traversals(0, &mut pre, &mut post, tree);
    (pre, post)
}

fn components(graph: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut visited = vec![false; graph.len()];
    let mut components = Vec::new();

    for start in 0..graph.len() {
        if visited[start] {
            continue;
        }

        visited[start] = true;
        let mut run = vec![start];
        let mut cursor = 0;

        while let Some(&u) = run.get(cursor) {
            cursor += 1;

            for &v in &graph[u] {
                if !visited[v] {
                    visited[v] = true;
                    run.push(v);
                }
            }
        }

        components.push(run)
    }

    components
}

fn main() {
    let mut buffer = String::new();
    let cases = read_line(&mut buffer);

    for case in 1..=cases {
        let (n, k) = read_line(&mut buffer);
        let tree: Vec<_> = read_lines_iter(&mut buffer, n)
            .map(|(l, r): (usize, usize)| [l.checked_sub(1), r.checked_sub(1)])
            .collect();

        let (pre, post) = order_traversals(&tree);
        let mut graph = vec![Vec::new(); n];

        for (u, v) in pre.into_iter().zip(post.into_iter()) {
            graph[u].push(v);
            graph[v].push(u);
        }

        let components = components(graph);

        if components.len() < k {
            println!("Case #{}: Impossible", case);
        } else {
            let mut labels = vec![0; n];

            for (label, component) in components.into_iter().enumerate() {
                for u in component {
                    labels[u] = label % k;
                }
            }

            print!("Case #{}:", case);

            for label in labels {
                print!(" {}", label + 1);
            }

            println!()
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
