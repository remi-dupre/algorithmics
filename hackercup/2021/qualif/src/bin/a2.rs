use std::cmp::min;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};
use testcase_derive::{hackercup, TestCase};

const LETTERS: usize = 26;

#[derive(Debug, Default)]
struct LetterMap<T>([T; LETTERS]);

impl<T> Index<u8> for LetterMap<T> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        assert!((b'A'..=b'Z').contains(&index));
        &self.0[(index - b'A') as usize]
    }
}

impl<T> IndexMut<u8> for LetterMap<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        assert!((b'A'..=b'Z').contains(&index));
        &mut self.0[(index - b'A') as usize]
    }
}

fn build_substitutions(edges: impl Iterator<Item = (u8, u8)>) -> LetterMap<LetterMap<Option<u16>>> {
    let mut substitutions = LetterMap::<LetterMap<_>>::default();

    for c in b'A'..=b'Z' {
        substitutions[c][c] = Some(0);
    }

    for (u, v) in edges {
        substitutions[u][v] = Some(1);
    }

    for k in b'A'..=b'Z' {
        for u in b'A'..=b'Z' {
            for v in b'A'..=b'Z' {
                if let (Some(w1), Some(w2)) = (substitutions[u][k], substitutions[k][v]) {
                    substitutions[u][v] =
                        Some(min(substitutions[u][v].unwrap_or(u16::MAX), w1 + w2));
                }
            }
        }
    }

    substitutions
}

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    word: String,
    #[testcase(line)]
    _n: usize,
    #[testcase(lines = "_n")]
    substitutions: Vec<String>,
}

#[hackercup(input = "../../data/a2/simple.in", output = "../../data/a2/simple.out")]
fn solve(input: Input) -> i32 {
    let substitutions = build_substitutions(input.substitutions.iter().map(|sub| {
        let [x, y]: [u8; 2] = sub
            .as_bytes()
            .try_into()
            .expect("edges should be two bytes long");
        (x, y)
    }));

    (b'A'..=b'Z')
        .filter_map(|target| {
            (input.word.bytes())
                .map(|c| substitutions[c][target])
                .sum::<Option<u16>>()
        })
        .min()
        .map(i32::from)
        .unwrap_or(-1)
}
