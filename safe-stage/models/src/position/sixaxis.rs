use maths::Vector3;
use std::f64::consts;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SixAxis {
    pub pos: Vector3,
    pub rot: Vector3,
}

impl Debug for SixAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coordinates")
            .field("x", &self.pos.x())
            .field("y", &self.pos.y())
            .field("z", &self.pos.z())
            .field(
                "rx",
                &format_args!("{} ({})", self.rot.x(), self.rot.x().to_degrees()),
            )
            .field(
                "ry",
                &format_args!("{} ({})", self.rot.y(), self.rot.y().to_degrees()),
            )
            .field(
                "rz",
                &format_args!("{} ({})", self.rot.z(), self.rot.z().to_degrees()),
            )
            .finish()
    }
}

impl Hash for SixAxis {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.x().to_bits().hash(state);
        self.pos.y().to_bits().hash(state);
        self.pos.z().to_bits().hash(state);
        self.rot.x().to_bits().hash(state);
        self.rot.y().to_bits().hash(state);
        self.rot.z().to_bits().hash(state);
    }
}

impl Add<&SixAxis> for &SixAxis {
    type Output = SixAxis;

    fn add(self, rhs: &SixAxis) -> Self::Output {
        SixAxis {
            pos: self.pos + rhs.pos,
            rot: self.rot + rhs.rot,
        }
    }
}

impl Sub<&SixAxis> for &SixAxis {
    type Output = SixAxis;
    fn sub(self, rhs: &SixAxis) -> Self::Output {
        SixAxis {
            pos: self.pos - rhs.pos,
            rot: self.shortest_rotation(rhs),
        }
    }
}

const TWO_PI: f64 = consts::PI * 2.0;

#[inline]
const fn angle_difference(a: f64, b: f64) -> f64 {
    let diff = (b - a + consts::PI) % TWO_PI - consts::PI;
    if diff < -consts::PI {
        diff + TWO_PI
    } else {
        diff
    }
}

impl SixAxis {
    #[inline]
    pub const fn from_rotation(rotation: Vector3) -> Self {
        SixAxis {
            pos: Vector3::ZERO,
            rot: rotation,
        }
    }

    #[inline]
    pub const fn from_position(position: Vector3) -> Self {
        SixAxis {
            pos: position,
            rot: Vector3::ZERO,
        }
    }

    #[deprecated]
    pub fn close_to_pos_rot(&self, other: &SixAxis, pos_e: f64, rotation_e: f64) -> bool {
        let pos_diff = self.pos - other.pos;
        pos_diff.x().abs() < pos_e
            && pos_diff.y().abs() < pos_e
            && pos_diff.z().abs() < pos_e
            && angle_difference(self.rot.x(), other.rot.x()).abs() < rotation_e
            && angle_difference(self.rot.y(), other.rot.y()).abs() < rotation_e
            && angle_difference(self.rot.z(), other.rot.z()).abs() < rotation_e
    }

    #[inline]
    pub fn close_to(&self, other: &SixAxis, e: &SixAxis) -> bool {
        let diff = (self - other).abs();
        diff.pos.x() < e.pos.x()
            && diff.pos.y() < e.pos.y()
            && diff.pos.z() < e.pos.z()
            && diff.rot.x() < e.rot.x()
            && diff.rot.y() < e.rot.y()
            && diff.rot.z() < e.rot.z()
    }

    #[deprecated]
    pub fn lerp(&self, other: &SixAxis, tm: f64, tr: f64) -> SixAxis {
        let tm = tm.clamp(0.0, 1.0);
        let tr = tr.clamp(0.0, 1.0);
        SixAxis {
            pos: self.pos.lerp(&other.pos, tm),
            rot: self.rot.lerp(&other.rot, tr),
        }
    }

    #[inline]
    pub fn lerp_t(&self, other: &SixAxis, t: f64) -> SixAxis {
        SixAxis {
            pos: self.pos.lerp(&other.pos, t),
            rot: self.rot.lerp(&other.rot, t),
        }
    }

    #[inline]
    pub fn abs(&self) -> SixAxis {
        SixAxis {
            pos: self.pos.abs(),
            rot: self.rot.abs(),
        }
    }

    #[deprecated]
    pub fn manhattan_distances(&self, other: &SixAxis) -> (f64, f64) {
        let pos_diff = (self.pos - other.pos).abs();
        let rot_diff = self.shortest_rotation(other).abs();
        (
            pos_diff.x() + pos_diff.y() + pos_diff.z(),
            rot_diff.x() + rot_diff.y() + rot_diff.z(),
        )
    }

    #[deprecated]
    pub fn euclidean_distances(&self, other: &SixAxis) -> (f64, f64) {
        let pos_diff = self.pos - other.pos;
        let rot_diff = self.shortest_rotation(other);
        (pos_diff.len(), rot_diff.len())
    }

    #[inline]
    pub fn euclidean_to(&self, other: &SixAxis) -> f64 {
        let d = other - self;
        d.magnitude()
    }

    #[inline]
    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn time_to(&self, other: &SixAxis, speed: &SixAxis) -> f64 {
        let pos = (self.pos - other.pos).abs();
        let rot = self.shortest_rotation(other);
        [
            pos.x() / speed.pos.x(),
            pos.y() / speed.pos.y(),
            pos.z() / speed.pos.z(),
            rot.x() / speed.rot.x(),
            rot.y() / speed.rot.y(),
            rot.z() / speed.rot.z(),
        ]
        .iter()
        .filter(|a| !a.is_nan())
        .map(|a| a.abs())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .map_or_else(
            || panic!("Cannot determine time_to: {self:?}, {other:?}, speed: {speed:?}"),
            |x| x,
        )
    }

    pub fn time_to_path(&self, path: &[SixAxis], speed: &SixAxis) -> f64 {
        path.windows(2)
            .map(|segment| self.time_to_segment(&segment[0], &segment[1], speed))
            .fold(f64::INFINITY, f64::min)
    }

    #[inline]
    pub fn time_to_segment(&self, start: &SixAxis, end: &SixAxis, speed: &SixAxis) -> f64 {
        let ab = (end - start).to_time(speed);
        let ap = (self - start).to_time(speed);
        let dot_ap_ab = ap.dot(&ab);
        let dot_ab_ab = ab.dot(&ab);
        let t = dot_ap_ab / dot_ab_ab;

        let closest = if t.is_nan() {
            *start
        } else {
            let t = t.clamp(0.0, 1.0);
            start.lerp_t(end, t)
        };

        self.time_to(&closest, speed)
    }

    #[inline]
    pub const fn shortest_rotation(&self, other: &SixAxis) -> Vector3 {
        Vector3::new(
            angle_difference(self.rot.x(), other.rot.x()),
            angle_difference(self.rot.y(), other.rot.y()),
            angle_difference(self.rot.z(), other.rot.z()),
        )
    }

    #[inline]
    pub const fn to_time(&self, speed: &SixAxis) -> SixAxis {
        SixAxis {
            pos: Vector3::new(
                self.pos.x() / speed.pos.x(),
                self.pos.y() / speed.pos.y(),
                self.pos.z() / speed.pos.z(),
            ),
            rot: Vector3::new(
                self.rot.x() / speed.rot.x(),
                self.rot.y() / speed.rot.y(),
                self.rot.z() / speed.rot.z(),
            ),
        }
    }

    #[inline]
    pub const fn to_distance(&self, speed: &SixAxis) -> SixAxis {
        SixAxis {
            pos: Vector3::new(
                self.pos.x() * speed.pos.x(),
                self.pos.y() * speed.pos.y(),
                self.pos.z() * speed.pos.z(),
            ),
            rot: Vector3::new(
                self.rot.x() * speed.rot.x(),
                self.rot.y() * speed.rot.y(),
                self.rot.z() * speed.rot.z(),
            ),
        }
    }

    #[inline]
    pub const fn dot(&self, other: &SixAxis) -> f64 {
        self.pos.x() * other.pos.x()
            + self.pos.y() * other.pos.y()
            + self.pos.z() * other.pos.z()
            + self.rot.x() * other.rot.x()
            + self.rot.y() * other.rot.y()
            + self.rot.z() * other.rot.z()
    }

    pub fn stepping(&self, other: &SixAxis, step: &SixAxis) -> usize {
        let diff = (other - self).abs();
        let steps = [
            diff.pos.x() / step.pos.x(),
            diff.pos.y() / step.pos.y(),
            diff.pos.z() / step.pos.z(),
            diff.rot.x() / step.rot.x(),
            diff.rot.y() / step.rot.y(),
            diff.rot.z() / step.rot.z(),
        ];

        let max_steps = steps
            .iter()
            .filter(|a| a.is_finite())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .expect("At least one step size must be finite");

        max_steps.ceil() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    fn r(rad: f64) -> f64 {
        rad.to_radians()
    }

    #[test]
    fn angle_distances_inverse() {
        let d = angle_difference(r(0.0), r(179.0));
        assert_float_absolute_eq!(r(179.0), d, 1e-10);

        let d = angle_difference(r(0.0), r(181.0));
        assert_float_absolute_eq!(-r(179.0), d, 1e-10);
    }

    #[test]
    fn angle_distances_rotations() {
        let d = angle_difference(r(0.0), r(899.0));
        assert_float_absolute_eq!(r(179.0), d, 1e-10);

        let d = angle_difference(r(0.0), r(901.0));
        assert_float_absolute_eq!(-r(179.0), d, 1e-10);

        let d = angle_difference(r(0.0), r(720.0));
        assert_float_absolute_eq!(r(0.0), d, 1e-10);
    }

    #[test]
    fn time_to_segment_middle() {
        let start = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::ZERO,
        };
        let end = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::new(0.0, r(180.0), 0.0),
        };
        let speed = SixAxis {
            pos: Vector3::new(100.0, 1.0, 100.0),
            rot: Vector3::new(100.0, 2.0, 100.0),
        };
        let point = SixAxis {
            pos: Vector3::new(0.0, -3.0, 0.0),
            rot: Vector3::new(0.0, r(90.0), 0.0),
        };

        //  Expected distance of 3 along Y axis
        let expected = 3.0;
        let actual = point.time_to_segment(&start, &end, &speed);
        assert_float_absolute_eq!(expected, actual, 1e-20);
    }

    #[test]
    fn time_to_segment_corner() {
        let start = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::ZERO,
        };
        let end = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::new(0.0, r(180.0), 0.0),
        };
        let speed = SixAxis {
            pos: Vector3::new(100.0, 1.0, 100.0),
            rot: Vector3::new(100.0, 2.0, 100.0),
        };
        let point = SixAxis {
            pos: Vector3::new(0.0, -3.0, 0.0),
            rot: Vector3::new(0.0, r(190.0), 0.0),
        };

        //  Expected distance of 3 along Y axis
        //  and distance of 10 degrees around Y axis
        //  (whichever takes longer time)
        let expected = 3.0_f64.max(r(10.0) / 2.0);
        let actual = point.time_to_segment(&start, &end, &speed);
        assert_float_absolute_eq!(expected, actual, 1e-20);
    }

    #[test]
    fn exclusive_stepping() {
        let start = SixAxis {
            pos: Vector3::ZERO,
            rot: Vector3::ZERO,
        };

        let end = SixAxis {
            pos: Vector3::new(1.0, 1.0, 1.0),
            rot: Vector3::ZERO,
        };

        let step = SixAxis {
            pos: Vector3::new(0.09, 0.13, 0.19),
            rot: Vector3::ZERO,
        };

        //  X: .00 -> .09 -> .18 -> .27 -> .36 -> .45 -> .54 -> .63 -> .72 -> .81 -> .90 -> .99 -> 1.08
        //                                                                                     ˄
        let expected = 12;
        let actual = start.stepping(&end, &step);
        assert_eq!(expected, actual);
    }
}
