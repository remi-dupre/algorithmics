use fxhash::FxHashMap;

fn get_sum(set: &[usize], sum_size: usize) -> usize {
    // sums[nb][x] is defined if `x` is the sum of `nb` elements read from from
    // the set so far, and it is set to `true` if only one such sum is known.
    let mut sums: Vec<FxHashMap<usize, bool>> = vec![Default::default(); sum_size];

    for &x in set {
        for nb in (0..sum_size - 1).rev() {
            // prev = &sums[nb], next = &mut sums[nb + 1]
            let (prev, next) = {
                let (head, tail) = sums.split_at_mut(nb + 1);
                (&head[nb], &mut tail[0])
            };

            for (y, unique) in prev {
                next.entry(x + y)
                    .and_modify(|unique| *unique = false)
                    .or_insert(*unique);
            }
        }

        sums[0].insert(x, true);
    }

    sums[sum_size - 1]
        .iter()
        .filter(|(_, unique)| **unique)
        .map(|(x, _)| x)
        .sum()
}

pub fn solve() -> usize {
    let set: Vec<usize> = (1..=100).map(|x| x * x).collect();
    get_sum(&set, 50)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_sum() {
        assert_eq!(get_sum(&[1, 3, 6, 8, 10, 11], 3), 156);
    }
}
