use std::num::ParseIntError;

pub fn generator(input: &str) -> Result<Vec<u16>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn get_all_nodes(adapters: &[u16]) -> Vec<u16> {
    let mut adapters = adapters.to_vec();
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort_unstable();
    adapters
}

pub fn part_1(adapters: &[u16]) -> usize {
    let adapters = get_all_nodes(adapters);
    let diff_1 = adapters.array_windows().filter(|[x, y]| y - x == 1).count();
    let diff_3 = adapters.array_windows().filter(|[x, y]| y - x == 3).count();
    diff_1 * diff_3
}

pub fn part_2(adapters: &[u16]) -> u64 {
    // Put adapters in decreasing order, this will be used to check for
    // ownership during our linear run.
    let mut adapters = get_all_nodes(adapters);
    adapters.reverse();
    adapters.pop();
    let mut prev = [0, 0, 1];

    for i in 1..=*adapters.first().unwrap() {
        if Some(&i) == adapters.last() {
            adapters.pop();
            prev = [prev[1], prev[2], prev.iter().sum()];
        } else {
            prev = [prev[1], prev[2], 0];
        }
    }

    prev[2]
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day10::*;

    const EXAMPLE: &str = crate::lines! {
        "28" "33" "18" "42" "31" "14" "46" "20" "48" "47" "24" "23" "49" "45" "19" "38"
            "39" "11" "1" "32" "25" "35" "8" "17" "7" "9" "4" "2" "34" "10" "3"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(220, part_1(&generator(EXAMPLE).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(19208, part_2(&generator(EXAMPLE).unwrap()));
    }
}
