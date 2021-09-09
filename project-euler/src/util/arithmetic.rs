use std::cmp::Ord;
use std::convert::TryInto;
use std::fmt::Debug;
use std::ops::{Div, DivAssign, Rem, Sub};

use num_bigint::BigUint;
use num_integer::Roots;
use num_traits::{One, Pow, Zero};

// ---
// --- Factorial
// ---

pub trait Factorial {
    fn factorial(self) -> Self;
}

macro_rules! impl_factorial_for_primitives {
    ( $type: ty ) => {
        impl Factorial for $type {
            fn factorial(self) -> Self {
                (1..=self).product()
            }
        }
    };
    ( $( $type: ty ),* ) => {
        $( impl_factorial_for_primitives!($type); )*
    };
}

impl_factorial_for_primitives!(u8, u16, u32, u64, u128, usize);

impl Factorial for BigUint {
    fn factorial(mut self) -> Self {
        let mut result = Self::one();

        while !self.is_zero() {
            result *= &self;
            self -= Self::one();
        }

        result
    }
}

// ---
// --- NbDigits
// ---

pub trait NbDigits {
    type Output;
    fn nb_digits(&self, base: u8) -> Self::Output;
}

macro_rules! impl_nb_digits_for_primitives {
    ( $type: ty ) => {
        impl NbDigits for $type {
            type Output = u8;

            fn nb_digits(&self, base: u8) -> Self::Output {
               1 + ((*self as f64).log2() / (base as f64).log2()) as u8
            }
        }
    };
    ( $( $type: ty ),* ) => {
        $( impl_nb_digits_for_primitives!($type); )*
    };
}

impl_nb_digits_for_primitives!(u8, u16, u32, u64, u128, usize);

impl NbDigits for BigUint {
    type Output = u64;

    fn nb_digits(&self, base: u8) -> Self::Output {
        1 + (self.bits() as f64 / f64::from(base).log2()) as u64
    }
}

// ---
// --- Digits
// ---

pub trait Digits {
    fn digits(&self, base: u8) -> Box<dyn Iterator<Item = u8> + '_>;
    fn digits_rev(&self, base: u8) -> Box<dyn Iterator<Item = u8> + '_>;
}

impl<T> Digits for T
where
    T: Clone
        + DivAssign<Self>
        + Div<Self, Output = Self>
        + From<u8>
        + TryInto<u8>
        + NbDigits
        + One
        + Ord
        + Pow<<Self as NbDigits>::Output, Output = Self>
        + Rem<Self, Output = Self>
        + Zero,
    <T as TryInto<u8>>::Error: Debug,
    <T as NbDigits>::Output: One + Sub<<T as NbDigits>::Output, Output = <T as NbDigits>::Output>,
{
    fn digits(&self, base: u8) -> Box<dyn Iterator<Item = u8> + '_> {
        debug_assert!(base > 0);

        if self.is_zero() {
            return Box::new(std::iter::once(0u8));
        }

        let mut curr_digit = Self::from(base).pow(self.nb_digits(base) - One::one());

        Box::new(std::iter::from_fn(move || {
            if !curr_digit.is_zero() {
                let res = ((self.clone() / curr_digit.clone()) % Self::from(base))
                    .try_into()
                    .expect("digits should fit into u8");
                curr_digit /= base.into();
                Some(res)
            } else {
                None
            }
        })) as _
    }

    fn digits_rev(&self, base: u8) -> Box<dyn Iterator<Item = u8> + '_> {
        debug_assert!(base > 0);

        if self.is_zero() {
            return Box::new(std::iter::once(0u8));
        }

        let mut curr = self.clone();

        Box::new(std::iter::from_fn(move || {
            if !curr.is_zero() {
                let res = curr.clone() % Self::from(base);
                curr /= base.into();
                Some(res.try_into().expect("digits should fit into u8"))
            } else {
                None
            }
        }))
    }
}

// ---
// --- Divisors
// ---

pub trait Divisors {
    fn divisors(self) -> Box<dyn Iterator<Item = Self>>;
}

macro_rules! impl_divisors_for_primitives {
    () => {};
    ( $type: ty $( , $other: ty )* ) => {
        impl Divisors for $type {
            fn divisors(self: $type) -> Box<dyn Iterator<Item = $type>> {
                let sqrt = self.sqrt();

                Box::new(
                    ((1..=sqrt).filter(move |d| self % d == 0))
                        .chain({
                            if self % sqrt == 0 && sqrt * sqrt != self {
                                Some(self / sqrt)
                            } else {
                                None
                            }
                        })
                        .chain(
                            (1..sqrt)
                                .rev()
                                .filter(move |d| self % d == 0)
                                .map(move |d| self / d),
                        ),
                )
            }
        }

        impl_divisors_for_primitives!($( $other ),*);
    };
}

impl_divisors_for_primitives!(u8, u16, u32, u64, u128, usize);

// ---
// --- Tests
// ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nb_digits() {
        assert_eq!(0u32.nb_digits(10), 1);
        assert_eq!(9u32.nb_digits(10), 1);
        assert_eq!(10u32.nb_digits(10), 2);
        assert_eq!(19u32.nb_digits(10), 2);
        assert_eq!(1_000_000_u32.nb_digits(10), 7);
        assert_eq!(0b0_u32.nb_digits(2), 1);
        assert_eq!(0b111_u32.nb_digits(2), 3);
    }

    #[test]
    fn test_digits() {
        assert_eq!(0u64.digits(10).collect::<Vec<_>>(), [0]);
        assert_eq!(156u8.digits(10).collect::<Vec<_>>(), [1, 5, 6]);
        assert_eq!(1056u16.digits(10).collect::<Vec<_>>(), [1, 0, 5, 6]);
        assert_eq!(180956u32.digits(10).collect::<Vec<_>>(), [1, 8, 0, 9, 5, 6]);
        assert_eq!(180956u64.digits(10).collect::<Vec<_>>(), [1, 8, 0, 9, 5, 6]);
        assert_eq!(18095usize.digits(10).collect::<Vec<_>>(), [1, 8, 0, 9, 5]);
        assert_eq!(
            BigUint::from(180956u64).digits(10).collect::<Vec<_>>(),
            [1, 8, 0, 9, 5, 6]
        );
    }

    #[test]
    fn test_digits_rev() {
        assert_eq!(0u64.digits_rev(10).collect::<Vec<_>>(), [0]);
        assert_eq!(156u8.digits_rev(10).collect::<Vec<_>>(), [6, 5, 1]);
        assert_eq!(1056u16.digits_rev(10).collect::<Vec<_>>(), [6, 5, 0, 1]);
        assert_eq!(
            180956u32.digits_rev(10).collect::<Vec<_>>(),
            [6, 5, 9, 0, 8, 1]
        );
        assert_eq!(
            180956u64.digits_rev(10).collect::<Vec<_>>(),
            [6, 5, 9, 0, 8, 1]
        );
        assert_eq!(
            18095usize.digits_rev(10).collect::<Vec<_>>(),
            [5, 9, 0, 8, 1]
        );
        assert_eq!(
            BigUint::from(180956u64).digits_rev(10).collect::<Vec<_>>(),
            [6, 5, 9, 0, 8, 1]
        );
    }
}
