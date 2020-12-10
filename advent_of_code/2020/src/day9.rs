use std::num::ParseIntError;

pub fn generator(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

pub fn find_pairs(x: u64, slice: &[u64]) -> impl Iterator<Item = (u64, u64)> + '_ {
    slice
        .iter()
        .enumerate()
        .flat_map(move |(i, &y)| slice[..i].iter().map(move |&z| (y, z)))
        .filter(move |(y, z)| x == y + z)
}

pub fn part_1(nums: &[u64]) -> Option<u64> {
    nums.array_windows::<26>()
        .find(|[prev @ .., x]| find_pairs(*x, prev).next().is_none())
        .map(|[.., x]| *x)
}

pub fn part_2(nums: &[u64]) -> Option<u64> {
    let target = part_1(nums)?;

    let slice = {
        let mut start = 0;
        let mut end = 0;
        let mut sum = 0;

        while sum != target {
            if sum > target {
                sum -= nums[start];
                start += 1;
            } else if end < nums.len() {
                sum += nums[end];
                end += 1;
            } else {
                return None;
            }
        }

        &nums[start..end]
    };

    Some(slice.iter().min()? + slice.iter().max()?)
}
