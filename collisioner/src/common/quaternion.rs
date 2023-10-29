use crate::common::Vector3;
use std::ops::Mul;

/// # Quaternion
/// Struct representing a quaternion.
///
/// ## Example
/// ```
/// use collisioner::common::Quaternion;
/// use collisioner::common::Vector3;
///
/// let euler = Vector3::new(30_f64.to_radians(), 45_f64.to_radians(), 90_f64.to_radians());
/// let quaternion = Quaternion::from_euler(euler);
///
/// assert_eq!(quaternion.x(), -0.09229595564125731);
/// assert_eq!(quaternion.y(), 0.4304593345768794);
/// assert_eq!(quaternion.z(), 0.560985526796931);
/// assert_eq!(quaternion.w(), 0.7010573846499779);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn from_euler(euler: Vector3) -> Self {
        let half = euler / 2.0;

        let rc = half.x().cos();
        let rs = half.x().sin();
        let pc = half.y().cos();
        let ps = half.y().sin();
        let yc = half.z().cos();
        let ys = half.z().sin();

        let x = rs * pc * yc - rc * ps * ys;
        let y = rc * ps * yc + rs * pc * ys;
        let z = rc * pc * ys - rs * ps * yc;
        let w = rc * pc * yc + rs * ps * ys;

        Self::new(x, y, z, w)
    }

    pub fn to_euler(self) -> Vector3 {
        let t0 = 2.0 * (self.w * self.x + self.y * self.z);
        let t1 = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let x = t0.atan2(t1);

        let t2 = 2.0 * (self.w * self.y - self.z * self.x);
        let t2 = if t2 > 1.0 {
            1.0
        } else if t2 < -1.0 {
            -1.0
        } else {
            t2
        };
        let y = t2.asin();

        let t3 = 2.0 * (self.w * self.z + self.x * self.y);
        let t4 = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let z = t3.atan2(t4);

        Vector3::new(x, y, z)
    }

    pub fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }
}

impl From<&Vector3> for Quaternion {
    fn from(vector: &Vector3) -> Self {
        Self::new(vector.x(), vector.y(), vector.z(), 0.0)
    }
}

impl From<Vector3> for Quaternion {
    fn from(vector: Vector3) -> Self {
        Self::from(&vector)
    }
}

impl Into<Vector3> for Quaternion {
    fn into(self) -> Vector3 {
        Vector3::new(self.x(), self.y(), self.z())
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Self::Output {
        Self {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn create_correct() {
        let quaternion = Quaternion::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(1.0, quaternion.x());
        assert_eq!(2.0, quaternion.y());
        assert_eq!(3.0, quaternion.z());
        assert_eq!(4.0, quaternion.w());
    }

    #[test]
    fn from_euler_angles() {
        let vector = Vector3::new(
            30_f64.to_radians(),
            45_f64.to_radians(),
            90_f64.to_radians(),
        );
        let quaternion = Quaternion::from_euler(vector);

        assert_eq!(-0.09229595564125731, quaternion.x());
        assert_eq!(0.4304593345768794, quaternion.y());
        assert_eq!(0.560985526796931, quaternion.z());
        assert_eq!(0.7010573846499779, quaternion.w());
    }

    #[test]
    fn into_euler_angles() {
        let quaternion = Quaternion::new(
            -0.09229595564125731,
            0.4304593345768794,
            0.560985526796931,
            0.7010573846499779,
        );
        let vector: Vector3 = quaternion.to_euler();

        assert_float_absolute_eq!(30.0, vector.x().to_degrees());
        assert_float_absolute_eq!(45.0, vector.y().to_degrees());
        assert_float_absolute_eq!(90.0, vector.z().to_degrees());
    }

    #[test]
    fn from_vector() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        let quaternion: Quaternion = vector.into();

        assert_eq!(1.0, quaternion.x());
        assert_eq!(2.0, quaternion.y());
        assert_eq!(3.0, quaternion.z());
        assert_eq!(0.0, quaternion.w());
    }

    #[test]
    fn into_vector() {
        let quaternion = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let vector: Vector3 = quaternion.into();

        assert_eq!(1.0, vector.x());
        assert_eq!(2.0, vector.y());
        assert_eq!(3.0, vector.z());
    }

    #[test]
    fn conjugate() {
        let quaternion = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let conjugate = quaternion.conjugate();

        assert_eq!(-1.0, conjugate.x());
        assert_eq!(-2.0, conjugate.y());
        assert_eq!(-3.0, conjugate.z());
        assert_eq!(4.0, conjugate.w());
    }

    #[test]
    fn multiply() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);
        let q3 = q1 * q2;

        assert_eq!(24.0, q3.x());
        assert_eq!(48.0, q3.y());
        assert_eq!(48.0, q3.z());
        assert_eq!(-6.0, q3.w());
    }
}
