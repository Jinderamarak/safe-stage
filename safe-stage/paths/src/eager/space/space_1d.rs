use bitvec::vec::BitVec;
use std::hint::unreachable_unchecked;

/// # 1D Grid Space
/// A 1D grid space with a given size and bounds represented by a bit vector.
pub struct Grid1DSpace {
    d: usize,
    min: f64,
    diff: f64,
    data: BitVec,
}

impl Grid1DSpace {
    pub fn new(d: usize, min: f64, max: f64) -> Self {
        let data = BitVec::repeat(true, d);
        Self {
            d,
            min,
            diff: (max - min).abs(),
            data,
        }
    }

    /// # Safety
    /// The data field is initialized to a capacity of 0 and will panic when accessing data.
    pub unsafe fn uninitialized(d: usize, min: f64, max: f64) -> Self {
        Self {
            d,
            min,
            diff: (max - min).abs(),
            data: BitVec::with_capacity(0),
        }
    }

    pub fn with_data(mut self, data: BitVec) -> Option<Self> {
        if data.len() != self.d {
            return None;
        }

        self.data = data;
        Some(self)
    }

    #[inline]
    pub const fn d(&self) -> usize {
        self.d
    }

    #[inline]
    pub fn global_to_grid(&self, global: f64) -> usize {
        ((global - self.min) / self.diff * (self.d - 1) as f64).round() as usize
    }

    #[inline]
    pub const fn grid_to_global(&self, local: usize) -> f64 {
        self.min + local as f64 / (self.d - 1) as f64 * self.diff
    }

    pub fn around_on_grid(&self, global: f64) -> [usize; 2] {
        let local = (global - self.min) / self.diff * (self.d - 1) as f64;
        let snapped = local as usize;
        if local % 1.0 < 0.5 {
            [snapped, snapped + 1]
        } else {
            [snapped + 1, snapped]
        }
    }

    #[inline]
    pub fn is_occupied(&self, x: usize) -> bool {
        self.data[x]
    }

    pub fn set_occupied(&mut self, x: usize) {
        self.data.set(x, true);
    }

    pub fn set_empty(&mut self, x: usize) {
        self.data.set(x, false);
    }

    #[inline]
    pub const fn neighbors_iter(&self, x: usize) -> NeighborsIter {
        NeighborsIter {
            state: NeighborsIterState::Init,
            x,
            space: self,
        }
    }
}

enum NeighborsIterState {
    Init,
    XPlus,
    XMinus,
    Finished,
}

impl NeighborsIterState {
    fn advance_state(&mut self) {
        *self = match self {
            Self::Init => Self::XPlus,
            Self::XPlus => Self::XMinus,
            Self::XMinus => Self::Finished,
            Self::Finished => Self::Finished,
        };
    }
}

pub struct NeighborsIter<'a> {
    state: NeighborsIterState,
    x: usize,
    space: &'a Grid1DSpace,
}

impl Iterator for NeighborsIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        use NeighborsIterState::*;

        self.state.advance_state();
        match self.state {
            Init => unsafe { unreachable_unchecked() },
            XPlus => {
                if self.x < self.space.d - 1 {
                    Some(self.x + 1)
                } else {
                    self.next()
                }
            }
            XMinus => {
                if self.x > 0 {
                    Some(self.x - 1)
                } else {
                    self.next()
                }
            }
            Finished => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_grid_to_global() {
        let space = Grid1DSpace::new(10, -1.0, 1.0);
        let grid = 0;

        let expected = -1.0;
        let actual = space.grid_to_global(grid);
        assert_eq!(expected, actual);
    }

    #[test]
    fn upper_grid_to_global() {
        let space = Grid1DSpace::new(10, -1.0, 1.0);
        let grid = 9;

        let expected = 1.0;
        let actual = space.grid_to_global(grid);
        assert_eq!(expected, actual);
    }

    #[test]
    fn lower_global_to_grid() {
        let space = Grid1DSpace::new(10, -1.0, 1.0);
        let global = -1.0;

        let expected = 0;
        let actual = space.global_to_grid(global);
        assert_eq!(expected, actual);
    }

    #[test]
    fn upper_global_to_grid() {
        let space = Grid1DSpace::new(10, -1.0, 1.0);
        let global = 1.0;

        let expected = 9;
        let actual = space.global_to_grid(global);
        assert_eq!(expected, actual);
    }

    #[test]
    fn global_to_grid_is_closest() {
        let space = Grid1DSpace::new(2, -1.0, 1.0);

        let global = 0.1;
        let expected = 1;
        let actual = space.global_to_grid(global);
        assert_eq!(expected, actual);

        let global = -0.1;
        let expected = 0;
        let actual = space.global_to_grid(global);
        assert_eq!(expected, actual);
    }

    #[test]
    fn around_on_grid_is_sorted() {
        let space = Grid1DSpace::new(3, -1.0, 1.0);

        let global = 0.3;
        let expected = [1, 2];
        let actual = space.around_on_grid(global);
        assert_eq!(expected, actual);

        let global = 0.5;
        let expected = [2, 1];
        let actual = space.around_on_grid(global);
        assert_eq!(expected, actual);
    }
}
