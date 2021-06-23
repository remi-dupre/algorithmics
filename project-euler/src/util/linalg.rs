use std::array::IntoIter;
use std::convert::{From, TryInto};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, Neg, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Vector<T, const N: usize>([T; N]);

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;

macro_rules! v {
    ( $( $x: expr ),* ) => { $crate::util::linalg::Vector::from([ $( $x ),* ]) }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn into_slice(self) -> [T; N] {
        self.0
    }

    pub fn as_slice(&self) -> &[T; N] {
        &self.0
    }

    pub fn as_slice_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(array: [T; N]) -> Self {
        Self(array)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Mul,
    T::Output: Sum,
{
    pub fn dot(self, rhs: Self) -> T::Output {
        (IntoIter::new(self.0).zip(IntoIter::new(rhs.0)))
            .map(|(x, y)| x * y)
            .sum()
    }
}

impl<T> Vector2<T> {
    pub fn x(&self) -> &T {
        &self.0[0]
    }

    pub fn y(&self) -> &T {
        &self.0[1]
    }
}

// Operators implementations

impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Neg, const N: usize> Neg for Vector<T, N> {
    type Output = Vector<T::Output, N>;

    fn neg(self) -> Self::Output {
        Vector(
            IntoIter::new(self.0)
                .map(|x| -x)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!("dimension inconsistency")),
        )
    }
}

impl<T: Add, const N: usize> Add for Vector<T, N> {
    type Output = Vector<T::Output, N>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(
            (IntoIter::new(self.0).zip(IntoIter::new(rhs.0)))
                .map(|(x, y)| x + y)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!("dimension inconsistency")),
        )
    }
}

impl<T: AddAssign, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for (x, y) in self.iter_mut().zip(IntoIter::new(rhs.0)) {
            *x += y;
        }
    }
}

impl<T: Sub, const N: usize> Sub for Vector<T, N> {
    type Output = Vector<T::Output, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(
            (IntoIter::new(self.0).zip(IntoIter::new(rhs.0)))
                .map(|(x, y)| x - y)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!("dimension inconsistency")),
        )
    }
}

impl<T: SubAssign, const N: usize> SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for (x, y) in self.iter_mut().zip(IntoIter::new(rhs.0)) {
            *x -= y;
        }
    }
}
