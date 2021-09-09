use qualif::c::Mine;
use testcase_derive::{hackercup, TestCase};

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    _n: usize,
    #[testcase(line)]
    weights: Vec<u64>,
    #[testcase(lines = "_n - 1")]
    edges: Vec<(usize, usize)>,
}

#[hackercup(input = "../../data/c1/simple.in", output = "../../data/c1/simple.out")]
fn solve(input: Input) -> u64 {
    let mine = Mine::new(input.weights, input.edges);
    mine.optimize(0, 1).ii.into_iter().max().unwrap()
}
