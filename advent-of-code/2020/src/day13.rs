use std::error::Error;
use std::num::ParseIntError;

pub struct Input {
    start: u64,
    lines: Vec<Option<u64>>,
}

pub fn generator(input: &str) -> Result<Input, Box<dyn Error>> {
    let start = input.lines().next().ok_or("empty input")?.parse()?;

    let lines = input
        .lines()
        .nth(1)
        .ok_or("missing busses starts")?
        .split(',')
        .map(|bus| {
            Ok::<_, ParseIntError>({
                if bus == "x" {
                    None
                } else {
                    Some(bus.parse()?)
                }
            })
        })
        .collect::<Result<_, _>>()?;

    Ok(Input { start, lines })
}

pub fn part_1(input: &Input) -> Option<u64> {
    let (wait_until, bus) = input
        .lines
        .iter()
        .filter_map(|&x| x)
        .map(|bus| (bus * ((input.start + bus - 1) / bus), bus))
        .min()?;

    Some((wait_until - input.start) * bus)
}

pub fn part_2(input: &Input) -> u64 {
    input
        .lines
        .iter()
        .enumerate()
        .filter_map(|(i, &bus)| Some((i as u64, bus?)))
        .fold((0, 1), |(mut t, jump), (offset, bus)| {
            while (t + offset) % bus != 0 {
                t += jump;
            }

            (t, jump * bus)
        })
        .0
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day13::*;

    const EXAMPLE: &str = crate::lines! {
        "939"
        "7,13,x,x,59,x,31,19"
    };

    const EXAMPLE_2_1: &str = crate::lines! {
        "939"
        "17,x,13,19"
    };

    const EXAMPLE_2_2: &str = crate::lines! {
        "939"
        "67,7,59,61"
    };

    const EXAMPLE_2_3: &str = crate::lines! {
        "939"
        "67,x,7,59,61"
    };

    const EXAMPLE_2_4: &str = crate::lines! {
        "939"
        "67,7,x,59,61"
    };

    const EXAMPLE_2_5: &str = crate::lines! {
        "939"
        "1789,37,47,1889"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(Some(295), part_1(&generator(EXAMPLE).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1068781, part_2(&generator(EXAMPLE).unwrap()));
        assert_eq!(3417, part_2(&generator(EXAMPLE_2_1).unwrap()));
        assert_eq!(754018, part_2(&generator(EXAMPLE_2_2).unwrap()));
        assert_eq!(779210, part_2(&generator(EXAMPLE_2_3).unwrap()));
        assert_eq!(1261476, part_2(&generator(EXAMPLE_2_4).unwrap()));
        assert_eq!(1202161486, part_2(&generator(EXAMPLE_2_5).unwrap()));
    }
}
