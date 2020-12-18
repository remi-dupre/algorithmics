use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::{Index, IndexMut};
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

    pub fn values_pos(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data
            .chunks(self.width())
            .enumerate()
            .flat_map(|(row, row_vals)| {
                row_vals
                    .iter()
                    .enumerate()
                    .map(move |(col, val)| (col, row, val))
            })
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

macro_rules! make_point_types {
    ($( $name: ident => $( $coord: ident ),+ ; )*) => {
        use std::hash::Hash;
        use std::ops::{Add, Mul, Sub};

        $(
            #[derive(Clone, Copy, Eq, Hash, PartialEq)]
            pub struct $name<T: Copy + Eq + Hash> {$(
                pub $coord: T,
            )+}

            impl<T: Copy + Eq + Hash> $name<T> {
                #[allow(dead_code)]
                pub fn new($($coord: T),+) -> Self {
                    Self { $($coord),+ }
                }
            }

            impl $name<i8> {
                #[allow(dead_code)]
                pub fn neighbours(&self) -> impl Iterator<Item = Self> {
                    let self_cpy = *self;
                    $( let mut $coord = 0; )+

                    std::iter::from_fn(move || {
                        let mut increment = || {
                            $(
                                // assigned values: 0 -> 1 -> -1 -> 0
                                $coord = ($coord + 2) % 3 - 1;

                                if $coord != 0 {
                                    return true;
                                }
                            )+

                            false
                        };

                        if increment() {
                            Some(Self {$(
                                $coord: self_cpy.$coord + $coord,
                            )+})
                        } else {
                            None
                        }
                    })
                }
            }

            impl<T: Copy + Eq + Hash + Mul> $name<T>
            where
                T::Output: Copy + Eq + Hash,
            {
                #[allow(dead_code)]
                pub fn mul(self, val: T) -> $name<T::Output> {
                    $name {$(
                        $coord: self.$coord * val,
                    )+}
                }
            }

            impl<T: Add + Copy + Eq + Hash> Add for $name<T>
            where
                T::Output: Copy + Eq + Hash,
            {
                type Output = $name<T::Output>;

                fn add(self, other: Self) -> Self::Output {
                    Self::Output {$(
                        $coord: self.$coord + other.$coord,
                    )+}
                }
            }

            impl<T: Copy + Eq + Hash + Sub> Sub for $name<T>
            where
                T::Output: Copy + Eq + Hash + Sub,
            {
                type Output = $name<T::Output>;

                fn sub(self, other: Self) -> Self::Output {
                    Self::Output {$(
                        $coord: self.$coord - other.$coord,
                    )+}
                }
            }
        )*
    }
}

make_point_types! {
    Point2D => x, y;
    Point3D => x, y, z;
    Point4D => x, y, z, w;
}
