use std::collections::HashMap;
use std::num::ParseIntError;

pub fn generator(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

pub fn nth_spoken(init: &[usize], nth: usize) -> Option<usize> {
    // Annonce a number and return the next to be said
    let mut say = {
        let mut said = HashMap::new();

        move |step: usize, val: usize| {
            if let Some(last_time) = said.insert(val, step) {
                step - last_time
            } else {
                0
            }
        }
    };

    let init_val = init.iter().enumerate().map(|(i, &x)| say(i, x)).last()?;
    Some((init.len()..(nth - 1)).fold(init_val, |val, step| say(step, val)))
}

pub fn part_1(init: &[usize]) -> Option<usize> {
    nth_spoken(init, 2020)
}

pub fn part_2(init: &[usize]) -> Option<usize> {
    nth_spoken(init, 30_000_000)
}

#[cfg(test)]
mod tests {
    use crate::day15::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(436), part_1(&generator("0,3,6").unwrap()));
        assert_eq!(Some(1), part_1(&generator("1,3,2").unwrap()));
        assert_eq!(Some(10), part_1(&generator("2,1,3").unwrap()));
        assert_eq!(Some(27), part_1(&generator("1,2,3").unwrap()));
        assert_eq!(Some(78), part_1(&generator("2,3,1").unwrap()));
        assert_eq!(Some(438), part_1(&generator("3,2,1").unwrap()));
        assert_eq!(Some(1836), part_1(&generator("3,1,2").unwrap()));
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(Some(175594), part_2(&generator("0,3,6").unwrap()));
    //     assert_eq!(Some(2578), part_2(&generator("1,3,2").unwrap()));
    //     assert_eq!(Some(3544142), part_2(&generator("2,1,3").unwrap()));
    //     assert_eq!(Some(261214), part_2(&generator("1,2,3").unwrap()));
    //     assert_eq!(Some(6895259), part_2(&generator("2,3,1").unwrap()));
    //     assert_eq!(Some(18), part_2(&generator("3,2,1").unwrap()));
    //     assert_eq!(Some(362), part_2(&generator("3,1,2").unwrap()));
    // }
}
