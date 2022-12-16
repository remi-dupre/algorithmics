use anyhow::{bail, Result};

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let cells = vec![default; width * height];

        Self {
            width,
            height,
            cells,
        }
    }
}

impl<T> Matrix<T> {
    pub fn try_from_iter(
        width: usize,
        height: usize,
        iter: impl Iterator<Item = Result<T>>,
    ) -> Result<Self> {
        let cells: Vec<_> = iter.take(width * height).collect::<Result<_>>()?;

        if cells.len() != width * height {
            bail!("iterator too short to build matrix")
        }

        Ok(Self {
            width,
            height,
            cells,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
        let positions = (0..self.height).flat_map(|y| (0..self.width).map(move |x| (x, y)));
        positions.zip(&self.cells)
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<&T> {
        let idx = self.index_for(pos)?;
        Some(&self.cells[idx])
    }

    pub fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut T> {
        let idx = self.index_for(pos)?;
        Some(&mut self.cells[idx])
    }

    pub fn get_line(&self, line: usize) -> Option<&[T]> {
        if line >= self.height {
            return None;
        }

        Some(&self.cells[(line * self.width)..((line + 1) * self.width)])
    }

    pub fn get_ptr_mut(&mut self, pos: (usize, usize)) -> Option<MatrixPtr<T>> {
        let inner = self.get_mut(pos)? as _;

        Some(MatrixPtr {
            matrix: self,
            inner,
        })
    }

    fn index_for(&self, (x, y): (usize, usize)) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.index_for_unchecked((x, y)))
    }

    fn index_to_coord(&self, idx: usize) -> Option<(usize, usize)> {
        let coord = (idx % self.width, idx / self.width);
        self.index_for(coord)?;
        Some(coord)
    }

    fn index_for_unchecked(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = self.index_for_unchecked(index);
        &self.cells[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.index_for_unchecked(index);
        &mut self.cells[idx]
    }
}

// Manipulate raw pointers on the matrix

pub struct MatrixPtr<'m, T> {
    matrix: &'m mut Matrix<T>,
    inner: *mut T,
}

impl<'m, T> MatrixPtr<'m, T> {
    pub fn as_pos(&self) -> Option<(usize, usize)> {
        if !self
            .matrix
            .cells
            .as_ptr_range()
            .contains(&(self.inner as _))
        {
            return None;
        }

        // We just checked that a valid array contains this ptr
        let idx = unsafe { self.inner.sub_ptr(self.matrix.cells.as_ptr()) };
        self.matrix.index_to_coord(idx)
    }

    pub fn set(&mut self, val: T) -> bool {
        if self.matrix.cells.as_mut_ptr_range().contains(&self.inner) {
            // We just checked that a valid array contains this ptr
            unsafe { self.inner.write(val) }
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> Option<&T> {
        if !self
            .matrix
            .cells
            .as_ptr_range()
            .contains(&(self.inner as _))
        {
            return None;
        }

        // We just checked that a valid array contains this ptr
        unsafe { self.inner.as_ref() }
    }

    pub fn add_rows(mut self, rows: usize) -> Self {
        self.inner = self.inner.wrapping_add(rows * self.matrix.width);
        self
    }

    pub fn sub_rows(mut self, rows: usize) -> Self {
        self.inner = self.inner.wrapping_sub(rows * self.matrix.width);
        self
    }

    pub fn add_cols(mut self, cols: usize) -> Self {
        self.inner = self.inner.wrapping_add(cols);
        self
    }

    pub fn sub_cols(mut self, cols: usize) -> Self {
        self.inner = self.inner.wrapping_sub(cols);
        self
    }
}
