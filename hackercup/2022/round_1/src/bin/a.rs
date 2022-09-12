use testcase_derive::{hackercup, TestCase};

fn kmp(word: &[u32]) -> Vec<isize> {
    let mut res = Vec::with_capacity(word.len());
    res.push(-1);

    for (i, &w) in word.iter().enumerate().skip(1) {
        let mut k = res[(i - 1) as usize] + 1;

        while k >= 0 && w != word[k as usize] {
            k = res[k as usize];
        }

        res.push(k);
    }

    res
}

fn contains(haysack: &[u32], needle: &[u32]) -> bool {
    if needle.is_empty() {
        return true;
    } else if haysack.is_empty() {
        return false;
    }

    let needle_kmp = kmp(needle);
    let mut k = -1;

    for &w in haysack {
        while k >= 0 && w != needle[(k + 1) as usize] {
            k = needle_kmp[k as usize];
        }

        if needle[(k + 1) as usize] == w {
            k += 1;
        }

        if k + 1 == needle.len() as isize {
            return true;
        }
    }

    false
}

#[derive(TestCase)]
struct Input {
    #[testcase(line)]
    n: usize,
    k: usize,
    #[testcase(line)]
    init: Vec<u32>,
    #[testcase(line)]
    excp: Vec<u32>,
}

#[hackercup(input = "../../data/a1/simple.in", output = "../../data/a1/simple.out")]
fn solve(input: Input) -> &'static str {
    let double_init = [&input.init[..], &input.init[..]].concat();

    let res = {
        if input.k == 0 || (input.n == 2 && input.k % 2 == 0) {
            input.init == input.excp
        } else if input.k == 1 || (input.n == 2 && input.k % 2 == 1) {
            contains(&double_init[1..double_init.len() - 1], &input.excp)
        } else {
            contains(&double_init, &input.excp)
        }
    };

    if res {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(contains(&[], &[]));
        assert!(contains(&[1], &[1]));
        assert!(contains(&[1, 2, 3], &[]));
        assert!(!contains(&[2], &[1]));
        assert!(!contains(&[1, 2, 1, 2, 1], &[1, 1]));
        assert!(!contains(&[1, 1], &[1, 1, 1]));
        assert!(!contains(&[1, 1, 2, 2, 3, 4], &[1, 2, 3]));
        assert!(contains(&[1, 2, 1, 1, 1, 2, 1, 1], &[1, 1, 2]));
        assert!(contains(&[2, 1, 2, 1, 2, 1, 3], &[2, 1, 2, 1, 3]));
    }
}
