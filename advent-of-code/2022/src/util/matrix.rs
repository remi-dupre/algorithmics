use std::ops::{Index, IndexMut, Range};

pub struct Matrix<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            width,
            height,
            cells: vec![default; width * height],
        }
    }
}

impl<T> Matrix<T> {
    pub fn get(&self, pos: (usize, usize)) -> Option<&T> {
        let idx = self.index_for(pos)?;
        Some(&self.cells[idx])
    }

    pub fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut T> {
        let idx = self.index_for(pos)?;
        Some(&mut self.cells[idx])
    }

    pub fn get_ptr_mut(&mut self, pos: (usize, usize)) -> Option<MatrixPtr<T>> {
        let inner = self.get_mut(pos)? as _;
        let range = self.cells.as_mut_ptr_range();

        Some(MatrixPtr {
            matrix: self,
            ptr_range: range,
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
    ptr_range: Range<*mut T>,
    inner: *mut T,
}

impl<'m, T> MatrixPtr<'m, T> {
    pub fn as_pos(&self) -> Option<(usize, usize)> {
        if !self.ptr_range.contains(&self.inner) {
            return None;
        }

        // We just checked that a valid array contains this ptr
        let idx = unsafe { self.inner.sub_ptr(self.ptr_range.start) };
        self.matrix.index_to_coord(idx)
    }

    pub fn set(&mut self, val: T) -> bool {
        if self.ptr_range.contains(&self.inner) {
            // We just checked that a valid array contains this ptr
            unsafe { self.inner.write(val) }
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> Option<&T> {
        if !self.ptr_range.contains(&self.inner) {
            return None;
        }

        // We just checked that a valid array contains this ptr
        unsafe { self.inner.as_ref() }
    }

    /// # Safety
    ///
    /// Ensure that your pointer is still pointing in the matrix while adding/removing some rows or
    /// columns.
    pub unsafe fn get_unchecked(&self) -> &T {
        &*self.inner
    }

    pub fn add_rows(&mut self, rows: usize) {
        self.inner = self.inner.wrapping_add(rows * self.matrix.width);
    }

    pub fn sub_rows(&mut self, rows: usize) {
        self.inner = self.inner.wrapping_sub(rows * self.matrix.width);
    }

    pub fn add_cols(&mut self, cols: usize) {
        self.inner = self.inner.wrapping_add(cols);
    }

    pub fn sub_cols(&mut self, cols: usize) {
        self.inner = self.inner.wrapping_sub(cols);
    }
}