use crate::neighbors::NeighborStrategy;
use maths::{Vector2, Vector3};
use models::position::sixaxis::SixAxis;

/// # No Rotation Grid
/// A neighbor strategy for a grid of dimensions `D` with constant rotation.
pub struct NoRotationGrid<const D: u8> {
    step: Vector3,
}

impl NoRotationGrid<3> {
    pub fn new(step: Vector3) -> Self {
        Self { step }
    }
}

impl NoRotationGrid<2> {
    pub fn new(step: Vector2) -> Self {
        Self {
            step: Vector3::new(step.x(), step.y(), 0.0),
        }
    }
}

impl NeighborStrategy<SixAxis> for NoRotationGrid<2> {
    #[inline]
    fn neighbors(&self, current: &SixAxis) -> impl Iterator<Item = SixAxis> + '_ {
        NeighborsIter::<3> {
            state: 0,
            current: *current,
            step: self.step,
        }
    }
}

impl NeighborStrategy<SixAxis> for NoRotationGrid<3> {
    #[inline]
    fn neighbors(&self, current: &SixAxis) -> impl Iterator<Item = SixAxis> + '_ {
        NeighborsIter::<3> {
            state: 0,
            current: *current,
            step: self.step,
        }
    }
}

pub struct NeighborsIter<const D: u8> {
    state: u8,
    current: SixAxis,
    step: Vector3,
}

impl Iterator for NeighborsIter<2> {
    type Item = SixAxis;

    fn next(&mut self) -> Option<Self::Item> {
        self.state += 1;
        match self.state {
            0 => unsafe { std::hint::unreachable_unchecked() },
            1 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x() + self.step.x(),
                    self.current.pos.y(),
                    0.0,
                ),
                rot: self.current.rot,
            }),
            2 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x() - self.step.x(),
                    self.current.pos.y(),
                    0.0,
                ),
                rot: self.current.rot,
            }),
            3 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y() + self.step.y(),
                    0.0,
                ),
                rot: self.current.rot,
            }),
            4 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y() - self.step.y(),
                    0.0,
                ),
                rot: self.current.rot,
            }),
            _ => {
                self.state -= 1;
                None
            }
        }
    }
}

impl Iterator for NeighborsIter<3> {
    type Item = SixAxis;

    fn next(&mut self) -> Option<Self::Item> {
        self.state += 1;
        match self.state {
            0 => unsafe { std::hint::unreachable_unchecked() },
            1 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x() + self.step.x(),
                    self.current.pos.y(),
                    self.current.pos.z(),
                ),
                rot: self.current.rot,
            }),
            2 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x() - self.step.x(),
                    self.current.pos.y(),
                    self.current.pos.z(),
                ),
                rot: self.current.rot,
            }),
            3 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y() + self.step.y(),
                    self.current.pos.z(),
                ),
                rot: self.current.rot,
            }),
            4 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y() - self.step.y(),
                    self.current.pos.z(),
                ),
                rot: self.current.rot,
            }),
            5 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y(),
                    self.current.pos.z() + self.step.z(),
                ),
                rot: self.current.rot,
            }),
            6 => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y(),
                    self.current.pos.z() - self.step.z(),
                ),
                rot: self.current.rot,
            }),
            _ => {
                self.state -= 1;
                None
            }
        }
    }
}
