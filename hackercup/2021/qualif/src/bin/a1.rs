use testcase_derive::hackercup;

fn is_vowel(c: u8) -> bool {
    "AEIOU".as_bytes().contains(&c)
}

fn cost_to_get_to(target: u8, word: &str) -> usize {
    word.bytes()
        .map(|c| {
            if target == c {
                0
            } else if is_vowel(target) != is_vowel(c) {
                1
            } else {
                2
            }
        })
        .sum()
}

#[hackercup(input = "../../data/a1/simple.in", output = "../../data/a1/simple.out")]
fn solve(word: String) -> usize {
    (b'A'..=b'Z')
        .map(|target| cost_to_get_to(target, word.trim()))
        .min()
        .expect("empty input")
}
