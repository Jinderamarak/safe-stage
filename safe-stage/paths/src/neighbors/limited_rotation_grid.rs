use crate::neighbors::NeighborStrategy;
use maths::Vector3;
use models::position::sixaxis::SixAxis;

/// # Limited Rotation Grid
/// A neighbor strategy for a grid. Generates neighbors for a given SixAxis position
/// with a rotation limited to the given range.
pub struct LimitedRotationGrid {
    step: SixAxis,
    start: Vector3,
    end: Vector3,
    closest: Vector3,
}

impl LimitedRotationGrid {
    pub fn new(step: SixAxis, start: Vector3, end: Vector3) -> Self {
        let first = SixAxis {
            pos: Vector3::ZERO,
            rot: start,
        };
        let second = SixAxis {
            pos: Vector3::ZERO,
            rot: start,
        };
        let diff = first.shortest_rotation(&second);
        let closest = start + diff;
        Self {
            step,
            start,
            end,
            closest,
        }
    }
}

impl NeighborStrategy<SixAxis> for LimitedRotationGrid {
    #[inline]
    fn neighbors(&self, current: &SixAxis) -> impl Iterator<Item = SixAxis> + '_ {
        NeighborsIter {
            state: NeighborsIterState::Init,
            strategy: self,
            current: *current,
            next: None,
        }
    }
}

#[inline]
fn rotation_axis_options(
    step: f64,
    current: f64,
    start: f64,
    end: f64,
    actual_end: f64,
) -> (Option<f64>, Option<f64>) {
    if step == 0.0 || start == end {
        return (None, None);
    }

    let lower = start.min(actual_end);
    let upper = start.max(actual_end);

    if current == lower {
        (Some(upper.min(current + step)), None)
    } else if current == upper {
        (Some(lower.max(current - step)), None)
    } else {
        (
            Some(lower.max(current - step)),
            Some(upper.min(current + step)),
        )
    }
}

enum NeighborsIterState {
    Init,
    PosXPlus,
    PosXMinus,
    PosYPlus,
    PosYMinus,
    PosZPlus,
    PosZMinus,
    RotX,
    RotY,
    RotZ,
    Finished,
}

impl NeighborsIterState {
    fn advance_state(&mut self) {
        *self = match self {
            Self::Init => Self::PosXPlus,
            Self::PosXPlus => Self::PosXMinus,
            Self::PosXMinus => Self::PosYPlus,
            Self::PosYPlus => Self::PosYMinus,
            Self::PosYMinus => Self::PosZPlus,
            Self::PosZPlus => Self::PosZMinus,
            Self::PosZMinus => Self::RotX,
            Self::RotX => Self::RotY,
            Self::RotY => Self::RotZ,
            Self::RotZ => Self::Finished,
            Self::Finished => Self::Finished,
        };
    }
}

pub struct NeighborsIter<'a> {
    state: NeighborsIterState,
    strategy: &'a LimitedRotationGrid,
    current: SixAxis,
    next: Option<SixAxis>,
}

impl Iterator for NeighborsIter<'_> {
    type Item = SixAxis;

    fn next(&mut self) -> Option<Self::Item> {
        use NeighborsIterState::*;

        if let Some(n) = self.next.take() {
            return Some(n);
        }

        self.state.advance_state();
        match self.state {
            Init => unsafe { std::hint::unreachable_unchecked() },
            PosXPlus => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x() + self.strategy.step.pos.x(),
                    self.current.pos.y(),
                    self.current.pos.z(),
                ),
                rot: self.current.rot,
            }),
            PosXMinus => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x() - self.strategy.step.pos.x(),
                    self.current.pos.y(),
                    self.current.pos.z(),
                ),
                rot: self.current.rot,
            }),
            PosYPlus => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y() + self.strategy.step.pos.y(),
                    self.current.pos.z(),
                ),
                rot: self.current.rot,
            }),
            PosYMinus => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y() - self.strategy.step.pos.y(),
                    self.current.pos.z(),
                ),
                rot: self.current.rot,
            }),
            PosZPlus => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y(),
                    self.current.pos.z() + self.strategy.step.pos.z(),
                ),
                rot: self.current.rot,
            }),
            PosZMinus => Some(SixAxis {
                pos: Vector3::new(
                    self.current.pos.x(),
                    self.current.pos.y(),
                    self.current.pos.z() - self.strategy.step.pos.z(),
                ),
                rot: self.current.rot,
            }),
            RotX => {
                match rotation_axis_options(
                    self.strategy.step.rot.x(),
                    self.current.rot.x(),
                    self.strategy.start.x(),
                    self.strategy.end.x(),
                    self.strategy.closest.x(),
                ) {
                    (None, None) => self.next(),
                    (Some(one), None) | (None, Some(one)) => Some(SixAxis {
                        pos: self.current.pos,
                        rot: Vector3::new(one, self.current.rot.y(), self.current.rot.z()),
                    }),
                    (Some(one), Some(two)) => {
                        self.next = Some(SixAxis {
                            pos: self.current.pos,
                            rot: Vector3::new(two, self.current.rot.y(), self.current.rot.z()),
                        });
                        Some(SixAxis {
                            pos: self.current.pos,
                            rot: Vector3::new(one, self.current.rot.y(), self.current.rot.z()),
                        })
                    }
                }
            }
            RotY => {
                match rotation_axis_options(
                    self.strategy.step.rot.y(),
                    self.current.rot.y(),
                    self.strategy.start.y(),
                    self.strategy.end.y(),
                    self.strategy.closest.y(),
                ) {
                    (None, None) => self.next(),
                    (Some(one), None) | (None, Some(one)) => Some(SixAxis {
                        pos: self.current.pos,
                        rot: Vector3::new(self.current.rot.x(), one, self.current.rot.z()),
                    }),
                    (Some(one), Some(two)) => {
                        self.next = Some(SixAxis {
                            pos: self.current.pos,
                            rot: Vector3::new(self.current.rot.x(), two, self.current.rot.z()),
                        });
                        Some(SixAxis {
                            pos: self.current.pos,
                            rot: Vector3::new(self.current.rot.x(), one, self.current.rot.z()),
                        })
                    }
                }
            }
            RotZ => {
                match rotation_axis_options(
                    self.strategy.step.rot.z(),
                    self.current.rot.z(),
                    self.strategy.start.z(),
                    self.strategy.end.z(),
                    self.strategy.closest.z(),
                ) {
                    (None, None) => self.next(),
                    (Some(one), None) | (None, Some(one)) => Some(SixAxis {
                        pos: self.current.pos,
                        rot: Vector3::new(self.current.rot.x(), self.current.rot.y(), one),
                    }),
                    (Some(one), Some(two)) => {
                        self.next = Some(SixAxis {
                            pos: self.current.pos,
                            rot: Vector3::new(self.current.rot.x(), self.current.rot.y(), two),
                        });
                        Some(SixAxis {
                            pos: self.current.pos,
                            rot: Vector3::new(self.current.rot.x(), self.current.rot.y(), one),
                        })
                    }
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
    fn is_within_regular_range() {
        let step = SixAxis {
            pos: Vector3::new(1.0, 1.0, 1.0),
            rot: Vector3::new(1.0, 1.0, 1.0),
        };
        let start = Vector3::new(0.0, 0.0, 10_f64.to_radians());
        let end = Vector3::new(0.0, 0.0, 30_f64.to_radians());

        let strategy = LimitedRotationGrid::new(step, start, end);

        let current = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::new(0.0, 0.0, 10_f64.to_radians()),
        };

        for neighbor in strategy.neighbors(&current) {
            assert_eq!(0.0, neighbor.rot.x());
            assert_eq!(0.0, neighbor.rot.y());
            assert!(neighbor.rot.z() >= 10_f64.to_radians());
        }

        let current = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::new(0.0, 0.0, 30_f64.to_radians()),
        };

        for neighbor in strategy.neighbors(&current) {
            assert_eq!(0.0, neighbor.rot.x());
            assert_eq!(0.0, neighbor.rot.y());
            assert!(neighbor.rot.z() <= 30_f64.to_radians());
        }
    }

    #[test]
    fn is_within_shorter_range() {
        let step = SixAxis {
            pos: Vector3::new(1.0, 1.0, 1.0),
            rot: Vector3::new(1.0, 1.0, 1.0),
        };

        // Shorter path between these two angles is going from 10deg to -90deg
        let start = Vector3::new(0.0, 0.0, 10_f64.to_radians());
        let end = Vector3::new(0.0, 0.0, 270_f64.to_radians());

        let strategy = LimitedRotationGrid::new(step, start, end);

        let current = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::new(0.0, 0.0, 10_f64.to_radians()),
        };

        for neighbor in strategy.neighbors(&current) {
            assert_eq!(0.0, neighbor.rot.x());
            assert_eq!(0.0, neighbor.rot.y());
            assert!(neighbor.rot.z() <= 10_f64.to_radians());
        }

        let current = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::new(0.0, 0.0, -90_f64.to_radians()),
        };

        for neighbor in strategy.neighbors(&current) {
            assert_eq!(0.0, neighbor.rot.x());
            assert_eq!(0.0, neighbor.rot.y());
            assert!(neighbor.rot.z() >= -90_f64.to_radians());
        }
    }
}
