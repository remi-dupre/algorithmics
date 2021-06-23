use rayon::prelude::*;

// x⁻¹ + y⁻¹ = n⁻¹
//     <=> x != n and y = (n⁻¹ - x⁻¹)⁻¹
//                      = nx / (x - n)
//
// ---
//
// n + 1 <= x, y <= n (n + 1):
//
//     nx / (x - n) >= (n + 1) <=> x (n - n - 1) >= -n (n + 1)
//                             <=> x <= n (n + 1)
//
//      x <= y <=> x <= nx / (x - n)
//             <=> x (x - n) <= nx
//             <=> x² - 2nx <= 0
//             <=> x (x - 2n) <= 0 (note: x > 0)
//             <=> x <= 2n

fn distinct_solutions(n: u64) -> usize {
    (n + 1..=2 * n).filter(|x| n * x % (x - n) == 0).count()
}

pub fn solve() -> u64 {
    (1..)
        .par_bridge()
        .find_first(|n| distinct_solutions(*n) > 1_000)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distinct_solutions() {
        assert_eq!(distinct_solutions(4), 3);
    }
}
