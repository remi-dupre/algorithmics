use testcase_derive::{hackercup, TestCase};

const MAX_PARTS: usize = 100;

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    _count_parts: usize,
    displays_capacity: usize,
    #[testcase(line)]
    parts: Vec<u8>,
}

fn count_repetitions(parts: &[u8]) -> [u8; MAX_PARTS] {
    let mut res = [0; 100];

    for &part in parts {
        res[(part - 1) as usize] += 1;
    }

    res
}

fn feasible(parts: &[u8], displays_capacity: usize) -> bool {
    let capacity_ok = parts.len() <= 2 * displays_capacity;

    let repetitions_ok = count_repetitions(&parts)
        .into_iter()
        .all(|count| count <= 2);

    capacity_ok && repetitions_ok
}

#[hackercup(input = "../../data/a/simple.in", output = "../../data/a/simple.out")]
fn solve(input: Input) -> &'static str {
    if feasible(&input.parts, input.displays_capacity) {
        "YES"
    } else {
        "NO"
    }
}
