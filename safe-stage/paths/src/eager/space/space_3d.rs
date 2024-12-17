use bitvec::vec::BitVec;
use maths::Vector3;
use std::hint::unreachable_unchecked;

pub struct Grid3DSpace {
    dx: usize,
    dy: usize,
    dz: usize,
    min: Vector3,
    diff: Vector3,
    data: BitVec,
}

impl Grid3DSpace {
    pub fn new(dx: usize, dy: usize, dz: usize, min: Vector3, max: Vector3) -> Self {
        let data = BitVec::repeat(true, dx * dy * dz);
        Self {
            dx,
            dy,
            dz,
            min,
            diff: (max - min).abs(),
            data,
        }
    }

    /// # Safety
    /// The data fields is initialized to capacity of 0 unusable data access.
    pub unsafe fn uninitialized(
        dx: usize,
        dy: usize,
        dz: usize,
        min: Vector3,
        max: Vector3,
    ) -> Self {
        Self {
            dx,
            dy,
            dz,
            min,
            diff: (max - min).abs(),
            data: BitVec::with_capacity(0),
        }
    }

    pub fn with_data(mut self, data: BitVec) -> Option<Self> {
        if data.len() != self.dx * self.dy * self.dz {
            return None;
        }

        self.data = data;
        Some(self)
    }

    #[inline]
    pub const fn dx(&self) -> usize {
        self.dx
    }

    #[inline]
    pub const fn dy(&self) -> usize {
        self.dy
    }

    #[inline]
    pub const fn dz(&self) -> usize {
        self.dz
    }

    #[inline]
    pub fn global_to_grid(&self, global: &Vector3) -> (usize, usize, usize) {
        (
            ((global.x() - self.min.x()) / self.diff.x() * (self.dx - 1) as f64).round() as usize,
            ((global.y() - self.min.y()) / self.diff.y() * (self.dy - 1) as f64).round() as usize,
            ((global.z() - self.min.z()) / self.diff.z() * (self.dz - 1) as f64).round() as usize,
        )
    }

    #[inline]
    pub const fn grid_to_global(&self, grid: &(usize, usize, usize)) -> Vector3 {
        Vector3::new(
            self.min.x() + grid.0 as f64 / (self.dx - 1) as f64 * self.diff.x(),
            self.min.y() + grid.1 as f64 / (self.dy - 1) as f64 * self.diff.y(),
            self.min.z() + grid.2 as f64 / (self.dz - 1) as f64 * self.diff.z(),
        )
    }

    pub fn around_on_grid(&self, global: &Vector3) -> [(usize, usize, usize); 8] {
        let (lx, ly, lz) = (
            ((global.x() - self.min.x()) / self.diff.x() * (self.dx - 1) as f64) as usize,
            ((global.y() - self.min.y()) / self.diff.y() * (self.dy - 1) as f64) as usize,
            ((global.z() - self.min.z()) / self.diff.z() * (self.dz - 1) as f64) as usize,
        );

        let (ux, uy, uz) = (
            ((global.x() - self.min.x()) / self.diff.x() * (self.dx - 1) as f64) as usize + 1,
            ((global.y() - self.min.y()) / self.diff.y() * (self.dy - 1) as f64) as usize + 1,
            ((global.z() - self.min.z()) / self.diff.z() * (self.dz - 1) as f64) as usize + 1,
        );

        let mut points = [
            (lx, ly, lz),
            (lx, ly, uz),
            (lx, uy, lz),
            (lx, uy, uz),
            (ux, ly, lz),
            (ux, ly, uz),
            (ux, uy, lz),
            (ux, uy, uz),
        ];

        points.sort_by(|a, b| {
            let ag = self.grid_to_global(a);
            let bg = self.grid_to_global(b);
            let a_dist = (ag - global).len();
            let b_dist = (bg - global).len();

            let cmp = a_dist.partial_cmp(&b_dist);
            if let Some(ord) = cmp {
                ord
            } else {
                log::debug!("a_dist: {a_dist:?} <> {b_dist:?} :b_dist");
                panic!("buh")
            }
        });

        points
    }

    #[inline]
    pub fn is_occupied(&self, x: usize, y: usize, z: usize) -> bool {
        self.data[x + y * self.dx + z * self.dx * self.dy]
    }

    pub fn set_occupied(&mut self, x: usize, y: usize, z: usize) {
        self.data.set(x + y * self.dx + z * self.dx * self.dy, true);
    }

    pub fn set_empty(&mut self, x: usize, y: usize, z: usize) {
        self.data
            .set(x + y * self.dx + z * self.dx * self.dy, false);
    }

    #[inline]
    pub const fn neighbors_iter(&self, x: usize, y: usize, z: usize) -> NeighborsIter {
        NeighborsIter {
            state: NeighborsIterState::Init,
            x,
            y,
            z,
            space: self,
        }
    }
}

enum NeighborsIterState {
    Init,
    XPlus,
    YPlus,
    ZPlus,
    XMinus,
    YMinus,
    ZMinus,
    Finished,
}

impl NeighborsIterState {
    fn advance_state(&mut self) {
        *self = match self {
            Self::Init => Self::XPlus,
            Self::XPlus => Self::YPlus,
            Self::YPlus => Self::ZPlus,
            Self::ZPlus => Self::XMinus,
            Self::XMinus => Self::YMinus,
            Self::YMinus => Self::ZMinus,
            Self::ZMinus => Self::Finished,
            Self::Finished => Self::Finished,
        };
    }
}

pub struct NeighborsIter<'a> {
    state: NeighborsIterState,
    x: usize,
    y: usize,
    z: usize,
    space: &'a Grid3DSpace,
}

impl Iterator for NeighborsIter<'_> {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        use NeighborsIterState::*;

        self.state.advance_state();
        match self.state {
            Init => unsafe { unreachable_unchecked() },
            XPlus => {
                if self.x < self.space.dx - 1 {
                    Some((self.x + 1, self.y, self.z))
                } else {
                    self.next()
                }
            }
            YPlus => {
                if self.y < self.space.dy - 1 {
                    Some((self.x, self.y + 1, self.z))
                } else {
                    self.next()
                }
            }
            ZPlus => {
                if self.z < self.space.dz - 1 {
                    Some((self.x, self.y, self.z + 1))
                } else {
                    self.next()
                }
            }
            XMinus => {
                if self.x > 0 {
                    Some((self.x - 1, self.y, self.z))
                } else {
                    self.next()
                }
            }
            YMinus => {
                if self.y > 0 {
                    Some((self.x, self.y - 1, self.z))
                } else {
                    self.next()
                }
            }
            ZMinus => {
                if self.z > 0 {
                    Some((self.x, self.y, self.z - 1))
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

    fn space() -> Grid3DSpace {
        Grid3DSpace::new(
            10,
            10,
            10,
            Vector3::new(-1.0, -2.0, -3.0),
            Vector3::new(1.0, 2.0, 3.0),
        )
    }

    #[test]
    fn lower_grid_to_global() {
        let s = space();
        let grid = (0, 0, 0);

        let expected = Vector3::new(-1.0, -2.0, -3.0);
        let actual = s.grid_to_global(&grid);
        assert_eq!(expected, actual);
    }

    #[test]
    fn upper_grid_to_global() {
        let s = space();
        let grid = (9, 9, 9);

        let expected = Vector3::new(1.0, 2.0, 3.0);
        let actual = s.grid_to_global(&grid);
        assert_eq!(expected, actual);
    }

    #[test]
    fn lower_global_to_grid() {
        let s = space();
        let global = Vector3::new(-1.0, -2.0, -3.0);

        let expected = (0, 0, 0);
        let actual = s.global_to_grid(&global);
        assert_eq!(expected, actual);
    }

    #[test]
    fn upper_global_to_grid() {
        let s = space();
        let global = Vector3::new(1.0, 2.0, 3.0);

        let expected = (9, 9, 9);
        let actual = s.global_to_grid(&global);
        assert_eq!(expected, actual);
    }

    #[test]
    fn global_to_grid_is_closest() {
        let space = Grid3DSpace::new(
            2,
            2,
            2,
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(1.0, 1.0, 1.0),
        );

        let global = Vector3::new(0.1, 0.1, 0.1);
        let expected = (1, 1, 1);
        let actual = space.global_to_grid(&global);
        assert_eq!(expected, actual);

        let global = Vector3::new(-0.1, -0.1, -0.1);
        let expected = (0, 0, 0);
        let actual = space.global_to_grid(&global);
        assert_eq!(expected, actual);
    }

    #[test]
    fn around_on_grid_is_sorted() {
        let space = Grid3DSpace::new(
            3,
            3,
            3,
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(1.0, 1.0, 1.0),
        );

        let global = Vector3::new(0.1, 0.3, 0.5);
        let expected = [
            (1, 1, 1),
            (1, 1, 2),
            (1, 2, 1),
            (1, 2, 2),
            (2, 1, 1),
            (2, 1, 2),
            (2, 2, 1),
            (2, 2, 2),
        ];
        let actual = space.around_on_grid(&global);

        assert_eq!(expected, actual);
    }
}
