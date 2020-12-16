use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::num::ParseIntError;

pub fn generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

pub fn nth_spoken(init: &[u32], nth: u32) -> Result<u32, Box<dyn Error>> {
    let mut say = {
        // Annonce a number and return the next to be said
        let mut said = vec![u32::MAX; usize::try_from(nth).expect("input must fit in usize")];

        move |step: u32, val: u32| {
            // let last_time = std::mem::replace(&mut said[usize::try_from(val)?], step);
            let last_time = std::mem::replace(&mut said[usize::try_from(val)?], step);
            Ok(step.saturating_sub(last_time))
        }
    };

    let init_val = init
        .iter()
        .enumerate()
        .map(|(i, &x)| say(i.try_into()?, x))
        .last()
        .ok_or("initialization is empty")??;

    (init.len().try_into()?..(nth - 1)).try_fold(init_val, |val, step| say(step, val))
}

pub fn part_1(init: &[u32]) -> Result<u32, Box<dyn Error>> {
    nth_spoken(init, 2020)
}

pub fn part_2(init: &[u32]) -> Result<u32, Box<dyn Error>> {
    nth_spoken(init, 30_000_000)
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day15::*;

    #[test]
    fn test_part_1() {
        assert_eq!(436, part_1(&generator("0,3,6").unwrap()).unwrap());
        assert_eq!(1, part_1(&generator("1,3,2").unwrap()).unwrap());
        assert_eq!(10, part_1(&generator("2,1,3").unwrap()).unwrap());
        assert_eq!(27, part_1(&generator("1,2,3").unwrap()).unwrap());
        assert_eq!(78, part_1(&generator("2,3,1").unwrap()).unwrap());
        assert_eq!(438, part_1(&generator("3,2,1").unwrap()).unwrap());
        assert_eq!(1836, part_1(&generator("3,1,2").unwrap()).unwrap());
    }
}
