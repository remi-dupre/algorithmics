use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::hash::Hash;

use rustc_hash::FxHashMap;

use crate::utils::{Matrix, Point3D, Point4D};

pub fn generator(input: &str) -> Result<Vec<(i8, i8)>, Box<dyn Error>> {
    let grid: Matrix<char> = input.parse()?;
    let mid_x = i8::try_from(grid.width())? / 2;
    let mid_y = i8::try_from(grid.height())? / 2;

    grid.values_pos()
        .filter(|(_, _, val)| **val == '#')
        .map(|(x, y, _)| {
            let x: i8 = x.try_into()?;
            let y: i8 = y.try_into()?;
            Ok((x - mid_x, y - mid_y))
        })
        .collect()
}

fn step<'i, T: Copy + Eq + Hash + 'i, I>(
    init: impl Iterator<Item = T> + 'i,
    neighbours: impl Fn(&T) -> I,
) -> impl Iterator<Item = T> + 'i
where
    I: Iterator<Item = T>,
{
    let mut weights = FxHashMap::<_, u8>::default();
    weights.reserve(3 * init.size_hint().1.unwrap_or(0));

    for pt in init {
        *weights.entry(pt).or_default() += 100;

        for other in neighbours(&pt) {
            *weights.entry(other).or_default() += 1;
        }
    }

    weights
        .into_iter()
        .filter(|(_, count)| [3, 102, 103].contains(&count))
        .map(|(pos, _)| pos)
}

fn run<'i, T: Copy + Eq + Hash + 'i, I>(
    iter: impl Iterator<Item = T> + 'i,
    neighbours: impl Copy + Fn(&T) -> I + 'i,
) -> impl Iterator<Item = T> + 'i
where
    I: Iterator<Item = T> + 'i,
{
    let iter = step(iter, neighbours);
    let iter = step(iter, neighbours);
    let iter = step(iter, neighbours);
    let iter = step(iter, neighbours);
    let iter = step(iter, neighbours);
    step(iter, neighbours)
}

pub fn part_1(init: &[(i8, i8)]) -> usize {
    run(
        init.iter().map(|&(x, y)| Point3D::new(x, y, 0)),
        Point3D::neighbours,
    )
    .count()
}

pub fn part_2(init: &[(i8, i8)]) -> usize {
    run(
        init.iter().map(|&(x, y)| Point4D::new(x, y, 0, 0)),
        Point4D::neighbours,
    )
    .count()
}

// ---
// --- Tests
// ---

#[cfg(test)]
mod tests {
    use crate::day17::*;

    const EXAMPLE: &str = crate::lines! {
        ".#."
        "..#"
        "###"
    };

    #[test]
    fn test_part_1() {
        assert_eq!(112, part_1(&generator(EXAMPLE).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(848, part_2(&generator(EXAMPLE).unwrap()));
    }
}
