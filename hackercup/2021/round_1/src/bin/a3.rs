use round_1::a::{sum_switches, Update};
use testcase_derive::{hackercup, TestCase};

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    _k: usize,
    #[testcase(line)]
    text: String,
}

// #[hackercup(input = "../../data/a2/simple.in", output = "../../data/a2/simple.out")]
#[hackercup]
fn solve(input: Input) -> u64 {
    let text = (input.text.bytes())
        .map(Update::try_from_byte)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse input string");

    sum_switches(&text)
}
