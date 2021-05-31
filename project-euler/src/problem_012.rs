use crate::util::arithmetic::divisors;

const MIN_COUNT: usize = 500;

fn triangles() -> impl Iterator<Item = u64> {
    (1..).scan(0, |sum, x| {
        *sum += x;
        Some(*sum)
    })
}

pub fn solve() -> u64 {
    triangles()
        .find(|x| divisors(*x).count() >= MIN_COUNT)
        .unwrap()
}
