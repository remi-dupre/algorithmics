#[allow(clippy::zero_prefixed_literal)]
const TRIANGLE: &[&[u64]] = &[
    &[75],
    &[95, 64],
    &[17, 47, 82],
    &[18, 35, 87, 10],
    &[20, 04, 82, 47, 65],
    &[19, 01, 23, 75, 03, 34],
    &[88, 02, 77, 73, 07, 63, 67],
    &[99, 65, 04, 28, 06, 16, 70, 92],
    &[41, 41, 26, 56, 83, 40, 80, 70, 33],
    &[41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
    &[53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
    &[70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
    &[91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
    &[63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
    &[04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
];

pub(crate) fn max_triangle_path(triangle: &[&[u64]]) -> Option<u64> {
    let init: Vec<_> = triangle.last()?.to_vec();

    triangle[..triangle.len() - 1]
        .iter()
        .rev()
        .fold(init, |prev, curr| {
            let prev = prev[1..]
                .iter()
                .zip(prev.iter())
                .map(|(l, r)| std::cmp::max(l, r));

            prev.zip(curr.iter())
                .map(|(prev_cost, cost)| prev_cost + cost)
                .collect()
        })
        .into_iter()
        .max()
}

pub fn solve() -> u64 {
    max_triangle_path(TRIANGLE).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_triangle_path() {
        assert_eq!(
            max_triangle_path(&[&[3], &[7, 4], &[2, 4, 6], &[8, 5, 9, 3]]),
            Some(23)
        );
    }
}
