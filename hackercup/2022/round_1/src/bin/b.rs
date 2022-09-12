use round_1::WrapU64;
use testcase_derive::{hackercup, TestCase};

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    _n: usize,
    #[testcase(lines = "_n")]
    trees: Vec<(WrapU64, WrapU64)>,
    #[testcase(line)]
    _q: usize,
    #[testcase(lines = "_q")]
    wells: Vec<(WrapU64, WrapU64)>,
}

/// The Polynom aX² - bX + c
struct Polynom {
    a: WrapU64,
    b: WrapU64,
    c: WrapU64,
}

impl Polynom {
    fn eval(&self, x: WrapU64) -> WrapU64 {
        self.a * x * x - self.b * x + self.c
    }
}

fn compile_trees(trees: &[(WrapU64, WrapU64)]) -> (Polynom, Polynom) {
    let a = u64::try_from(trees.len()).expect("too many wells").into();
    let b_x = WrapU64::from(2) * trees.iter().map(|w| w.0).sum();
    let b_y = WrapU64::from(2) * trees.iter().map(|w| w.1).sum();
    let c_x = trees.iter().map(|w| w.0 * w.0).sum();
    let c_y = trees.iter().map(|w| w.1 * w.1).sum();
    (Polynom { a, b: b_x, c: c_x }, Polynom { a, b: b_y, c: c_y })
}

#[hackercup(input = "../../data/b2/simple.in", output = "../../data/b2/simple.out")]
fn solve(input: Input) -> u64 {
    let (poly_x, poly_y) = compile_trees(&input.trees);
    let sum_x: WrapU64 = input.wells.iter().map(|(x, _)| poly_x.eval(*x)).sum();
    let sum_y: WrapU64 = input.wells.iter().map(|(_, y)| poly_y.eval(*y)).sum();
    (sum_x + sum_y).into()
}
