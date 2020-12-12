use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul};
use std::str::FromStr;

#[macro_export]
macro_rules! lines {
    ( $line: literal ) => {
        $line
    };
    ( $line: literal $( $tail: literal )+ ) => {
        concat!($line, "\n", $crate::lines!($( $tail )+))
    };
}

// Signed add

pub trait SignedAdd: Sized {
    type Signed;
    fn signed_add(self, other: Self::Signed) -> Option<Self>;
}

impl SignedAdd for usize {
    type Signed = isize;

    fn signed_add(self, other: Self::Signed) -> Option<Self> {
        if other >= 0 {
            self.checked_add(other.try_into().unwrap())
        } else {
            self.checked_sub((-other).try_into().unwrap())
        }
    }
}

// Inlined matrix

#[derive(Clone)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, width: usize) -> Self {
        let mut data = data.into();
        let height = data.len() / width;
        assert_eq!(data.len(), width * height);

        data.shrink_to_fit();
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width() || y >= self.height() {
            None
        } else {
            Some(unsafe { self.data.get_unchecked(x + y * self.width()) })
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width() || y >= self.height() {
            None
        } else {
            Some(unsafe { self.data.get_unchecked_mut(x + y * self.width) })
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = (usize, usize)> {
        let height = self.height();
        let width = self.width();
        (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get(x, y).expect("out of matrix bounds")
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_mut(x, y).expect("out of matrix bounds")
    }
}

impl<T: TryFrom<char>> FromStr for Matrix<T> {
    type Err = <T as TryFrom<char>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .map(|line| line.chars().count())
            .unwrap_or(0);

        let cells = s
            .lines()
            .flat_map(|line| line.chars().map(char::try_into))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Matrix::new(cells, width))
    }
}

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{:?}", self.get(x, y).unwrap())?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

// Point type

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy + Mul> Point<T> {
    pub fn mul(self, val: T) -> Point<T::Output> {
        Point {
            x: val * self.x,
            y: val * self.y,
        }
    }
}

impl<T: Add> Add for Point<T> {
    type Output = Point<T::Output>;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Clone> Clone for Point<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Copy> Copy for Point<T> {}
