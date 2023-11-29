use crate::Vector3;
use std::ops::Mul;

/// # Quaternion
/// Quaternion implementation useful for rotations in 3D space.
///
/// ## Example
/// ```
/// use assert_float_eq::*;
/// use mather::Quaternion;
/// use mather::Vector3;
///
/// let euler = Vector3::new(30_f64.to_radians(), 45_f64.to_radians(), 90_f64.to_radians());
/// let quaternion = Quaternion::from_euler(euler);
///
/// assert_float_absolute_eq!(quaternion.x(), -0.0922, 0.0001);
/// assert_float_absolute_eq!(quaternion.y(), 0.4304, 0.0001);
/// assert_float_absolute_eq!(quaternion.z(), 0.5609, 0.0001);
/// assert_float_absolute_eq!(quaternion.w(), 0.7010, 0.0001);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Quaternion {
    /// Creates a new quaternion from raw values.
    pub fn raw(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a new normalized quaternion from raw values.
    pub fn normalized(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self::raw(x, y, z, w).normalize()
    }

    /// Returns the quaternion's `x` component.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the quaternion's `y` component.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the quaternion's `z` component.
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Returns the quaternion's `w` component.
    pub fn w(&self) -> f64 {
        self.w
    }

    /// Creates a quaternion from euler angles according to the right hand thumb rule.
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

        Self::normalized(x, y, z, w)
    }

    /// Converts a quaternion to euler angles according to the right hand thumb rule.
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

    /// Returns the conjugate of the quaternion.
    pub fn conjugate(self) -> Self {
        Self::raw(-self.x, -self.y, -self.z, self.w)
    }

    /// Returns the length of the quaternion.
    pub fn len(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Returns the normalized quaternion.
    pub fn normalize(&self) -> Self {
        let len = self.len();
        if len == 0.0 {
            *self
        } else {
            Self::raw(self.x / len, self.y / len, self.z / len, self.w / len)
        }
    }
}

impl From<&Vector3> for Quaternion {
    fn from(vector: &Vector3) -> Self {
        Self::raw(vector.x(), vector.y(), vector.z(), 0.0)
    }
}

impl From<Vector3> for Quaternion {
    fn from(vector: Vector3) -> Self {
        Self::from(&vector)
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
    fn raw() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);

        assert_eq!(1.0, quaternion.x());
        assert_eq!(2.0, quaternion.y());
        assert_eq!(3.0, quaternion.z());
        assert_eq!(4.0, quaternion.w());
    }

    #[test]
    fn normalized() {
        let quaternion = Quaternion::normalized(1.0, 2.0, 3.0, 4.0);

        assert_float_absolute_eq!(0.0333, quaternion.x(), 1e-3);
        assert_float_absolute_eq!(0.0666, quaternion.y(), 1e-3);
        assert_float_absolute_eq!(0.0999, quaternion.z(), 1e-3);
        assert_float_absolute_eq!(0.1333, quaternion.w(), 1e-3);
    }

    #[test]
    fn from_euler_angles() {
        let vector = Vector3::new(
            30_f64.to_radians(),
            45_f64.to_radians(),
            90_f64.to_radians(),
        );
        let quaternion = Quaternion::from_euler(vector);

        assert_float_absolute_eq!(-0.0922, quaternion.x(), 0.0001);
        assert_float_absolute_eq!(0.4304, quaternion.y(), 0.0001);
        assert_float_absolute_eq!(0.5609, quaternion.z(), 0.0001);
        assert_float_absolute_eq!(0.7010, quaternion.w(), 0.0001);
    }

    #[test]
    fn to_euler_angles() {
        let quaternion = Quaternion::normalized(-0.0922, 0.4304, 0.5609, 0.7010);
        let vector: Vector3 = quaternion.to_euler();

        assert_float_absolute_eq!(30.0, vector.x().to_degrees(), 1.0);
        assert_float_absolute_eq!(45.0, vector.y().to_degrees(), 1.0);
        assert_float_absolute_eq!(90.0, vector.z().to_degrees(), 1.0);
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
    fn conjugate() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let conjugate = quaternion.conjugate();

        assert_eq!(-1.0, conjugate.x());
        assert_eq!(-2.0, conjugate.y());
        assert_eq!(-3.0, conjugate.z());
        assert_eq!(4.0, conjugate.w());
    }

    #[test]
    fn multiply() {
        let q1 = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::raw(5.0, 6.0, 7.0, 8.0);
        let q3 = q1 * q2;

        assert_eq!(24.0, q3.x());
        assert_eq!(48.0, q3.y());
        assert_eq!(48.0, q3.z());
        assert_eq!(-6.0, q3.w());
    }

    #[test]
    fn length() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let len = quaternion.len();

        assert_eq!(30.0, len);
    }

    #[test]
    fn normalize() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let normalized = quaternion.normalize();

        assert_float_absolute_eq!(0.0333, normalized.x(), 1e-3);
        assert_float_absolute_eq!(0.0666, normalized.y(), 1e-3);
        assert_float_absolute_eq!(0.0999, normalized.z(), 1e-3);
        assert_float_absolute_eq!(0.1333, normalized.w(), 1e-3);
    }

    #[test]
    fn normalize_zero_length() {
        let quaternion = Quaternion::raw(0.0, 0.0, 0.0, 0.0);
        let normalized = quaternion.normalize();

        assert_eq!(0.0, normalized.x());
        assert_eq!(0.0, normalized.y());
        assert_eq!(0.0, normalized.z());
        assert_eq!(0.0, normalized.w());
    }
}
