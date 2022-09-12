use round_1::WrapU64;
use testcase_derive::{hackercup, TestCase};

const M: u64 = 1_000_000_007;

type Wu64 = WrapU64<M>;

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    _n: usize,
    #[testcase(lines = "_n")]
    trees: Vec<(Wu64, Wu64)>,
    #[testcase(line)]
    _q: usize,
    #[testcase(lines = "_q")]
    wells: Vec<(Wu64, Wu64)>,
}

/// The Polynom aX² - bX + c
struct Polynom {
    a: Wu64,
    b: Wu64,
    c: Wu64,
}

impl Polynom {
    fn eval(&self, x: Wu64) -> Wu64 {
        self.a * x * x - self.b * x + self.c
    }
}

fn compile_trees(trees: &[(Wu64, Wu64)]) -> (Polynom, Polynom) {
    let a = u64::try_from(trees.len()).expect("too many wells").into();
    let b_x = Wu64::from(2) * trees.iter().map(|w| w.0).sum();
    let b_y = Wu64::from(2) * trees.iter().map(|w| w.1).sum();
    let c_x = trees.iter().map(|w| w.0 * w.0).sum();
    let c_y = trees.iter().map(|w| w.1 * w.1).sum();
    (Polynom { a, b: b_x, c: c_x }, Polynom { a, b: b_y, c: c_y })
}

#[hackercup(input = "../../data/b2/simple.in", output = "../../data/b2/simple.out")]
fn solve(input: Input) -> u64 {
    let (poly_x, poly_y) = compile_trees(&input.trees);
    let sum_x: Wu64 = input.wells.iter().map(|(x, _)| poly_x.eval(*x)).sum();
    let sum_y: Wu64 = input.wells.iter().map(|(_, y)| poly_y.eval(*y)).sum();
    (sum_x + sum_y).into()
}
