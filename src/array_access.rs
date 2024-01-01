/// Given an x, y, and width returns the index to access into a slice.
#[inline]
pub(crate) fn idx_xy(x: usize, y: usize, w: usize) -> usize {
    x + y * w
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Slice2D<'a, T> {
    data: &'a [T],
    width: usize,
    height: usize,
}

impl<'a, T> Slice2D<'a, T> {
    #[inline]
    pub fn new(data: &'a [T], width: usize, height: usize) -> Self {
        assert!(data.len() == width * height);
        Self {
            data,
            width,
            height,
        }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> &T {
        assert!(x < self.width);
        assert!(y < self.width);
        &self.data[idx_xy(x, y, self.width)]
    }

    #[inline]
    pub fn get_checked(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.get(x, y))
    }
    /// Returns 4-connected pixels adjacent to the given.
    /// [up, right, left, down]
    pub fn adj(&self, x: usize, y: usize) -> [Option<&T>; 4] {
        [
            self.get_checked(x, y + 1),
            self.get_checked(x + 1, y),
            x.checked_sub(1).map(|xm1| self.get(xm1, y)),
            y.checked_sub(1).map(|ym1| self.get(x, ym1)),
        ]
    }
}
