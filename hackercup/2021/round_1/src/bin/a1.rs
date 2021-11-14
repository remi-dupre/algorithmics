use round_1::a::{minimum_switches, Update};
use testcase_derive::{hackercup, TestCase};

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    _k: usize,
    #[testcase(line)]
    text: String,
}

#[hackercup(input = "../../data/a1/simple.in", output = "../../data/a1/simple.out")]
fn solve(input: Input) -> u64 {
    let text = (input.text.bytes())
        .map(Update::try_from_byte)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse input string");

    *minimum_switches(&text).last().unwrap()
}
