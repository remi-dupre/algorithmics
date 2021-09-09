use std::cmp::{Ord, Ordering};
use std::convert::From;
use std::ops::{Add, Mul};

use num_traits::Zero;

use crate::util::arithmetic::{Digits, Factorial};

// ---
// --- Permutations
// ---

pub fn next_permutation<T: Ord>(nums: &mut [T]) -> bool {
    if let Some(last_ascending) = nums.windows(2).rposition(|w| w[0] < w[1]) {
        let swap_with = nums[last_ascending + 1..]
            .binary_search_by(|n| nums[last_ascending].cmp(&n).then(Ordering::Less))
            .unwrap_err();
        nums.swap(last_ascending, last_ascending + swap_with);
        nums[last_ascending + 1..].reverse();
        true
    } else {
        nums.reverse();
        false
    }
}

pub trait Permutations {
    fn permutations(self) -> Box<dyn Iterator<Item = Self>>;
}

impl<T> Permutations for T
where
    T: Add<Self, Output = Self> + Mul<Self, Output = Self> + Clone + Digits + From<u8> + Zero,
{
    fn permutations(self) -> Box<dyn Iterator<Item = Self>> {
        let mut digits: Vec<_> = self.digits(10).collect();

        Box::new((0..(digits.len() as u128).factorial()).map(move |_| {
            next_permutation(&mut digits);

            digits
                .iter()
                .fold(Self::zero(), |sum, d| sum * Self::from(10) + Self::from(*d))
        }))
    }
}
