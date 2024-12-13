use crate::Vector3;
use std::ops::Mul;

/// # Quaternion
/// Quaternion implementation useful for rotations in 3D space.
///
/// ## Example
/// ```
/// use assert_float_eq::*;
/// use maths::Quaternion;
/// use maths::Vector3;
///
/// let euler = Vector3::new(30_f64.to_radians(), 45_f64.to_radians(), 90_f64.to_radians());
/// let quaternion = Quaternion::from_euler(&euler);
///
/// assert_float_absolute_eq!(quaternion.w(), 0.7010, 0.0001);
/// assert_float_absolute_eq!(quaternion.x(), -0.0922, 0.0001);
/// assert_float_absolute_eq!(quaternion.y(), 0.4304, 0.0001);
/// assert_float_absolute_eq!(quaternion.z(), 0.5609, 0.0001);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

//  Eq can be implemented since the constructor checks forbids NaN values
impl Eq for Quaternion {}

impl Quaternion {
    pub const IDENTITY: Quaternion = Quaternion::raw(1.0, 0.0, 0.0, 0.0);

    /// Creates a new quaternion from raw values.
    #[inline]
    pub const fn raw(w: f64, x: f64, y: f64, z: f64) -> Self {
        debug_assert!(!w.is_nan() && !x.is_nan() && !y.is_nan() && !z.is_nan());
        Self { w, x, y, z }
    }

    /// Creates a new normalized quaternion from raw values.
    #[inline]
    pub fn normalized(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self::raw(w, x, y, z).normalize()
    }

    /// Returns the quaternion's `w` component.
    #[inline]
    pub const fn w(&self) -> f64 {
        self.w
    }

    /// Returns the quaternion's `x` component.
    #[inline]
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Returns the quaternion's `y` component.
    #[inline]
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Returns the quaternion's `z` component.
    #[inline]
    pub const fn z(&self) -> f64 {
        self.z
    }

    /// Creates a quaternion from euler angles according to the right hand thumb rule.
    pub fn from_euler(euler: &Vector3) -> Self {
        let half = euler / 2.0;

        let rc = half.x().cos();
        let rs = half.x().sin();
        let pc = half.y().cos();
        let ps = half.y().sin();
        let yc = half.z().cos();
        let ys = half.z().sin();

        let w = rc * pc * yc + rs * ps * ys;
        let x = rs * pc * yc - rc * ps * ys;
        let y = rc * ps * yc + rs * pc * ys;
        let z = rc * pc * ys - rs * ps * yc;

        Self::normalized(w, x, y, z)
    }

    /// Converts a quaternion to euler angles according to the right hand thumb rule.
    pub fn to_euler(self) -> Vector3 {
        let t0 = 2.0 * (self.w * self.x + self.y * self.z);
        let t1 = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let x = t0.atan2(t1);

        let t2 = 2.0 * (self.w * self.y - self.z * self.x);
        let y = t2.clamp(-1.0, 1.0).asin();

        let t3 = 2.0 * (self.w * self.z + self.x * self.y);
        let t4 = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let z = t3.atan2(t4);

        Vector3::new(x, y, z)
    }

    /// Creates a quaternion from an axis and an angle.
    pub fn from_axis_angle(vector: &Vector3, angle: f64) -> Self {
        let sine = (angle / 2.0).sin();
        let xyz = vector.normalize() * sine;

        let cosine = (angle / 2.0).cos();
        Self::normalized(cosine, xyz.x(), xyz.y(), xyz.z())
    }

    /// Converts a quaternion to an axis and an angle.
    pub fn to_axis_angle(self) -> (Vector3, f64) {
        let div = (1.0 - self.w() * self.w()).sqrt();
        let x = self.x() / div;
        let y = self.y() / div;
        let z = self.z() / div;

        let angle = 2.0 * self.w().acos();
        let vector = Vector3::new(x, y, z);

        (vector, angle)
    }

    /// Returns the conjugate of the quaternion.
    #[inline]
    pub const fn conjugate(&self) -> Self {
        Self::raw(self.w, -self.x, -self.y, -self.z)
    }

    /// Returns the length of the quaternion.
    #[inline]
    pub fn len(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns the normalized quaternion.
    #[inline]
    pub fn normalize(&self) -> Self {
        let len = self.len();
        if len == 0.0 {
            *self
        } else {
            Self::raw(self.w / len, self.x / len, self.y / len, self.z / len)
        }
    }
}

macro_rules! from_vector3_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for Quaternion {
            #[inline]
            fn from(vector: $t) -> Self {
                Self::raw(0.0, vector.x(), vector.y(), vector.z())
            }
        }
    )*)
}

from_vector3_impl! { &Vector3 Vector3 }

macro_rules! mul_impl {
    ($($t1:ty, $t2:ty)*) => ($(
        impl Mul<$t2> for $t1 {
            type Output = Quaternion;

            #[inline]
            fn mul(self, other: $t2) -> Self::Output {
                Quaternion {
                    w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
                    x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
                    y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
                    z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
                }
            }
        }
    )*)
}

mul_impl! { Quaternion, Quaternion &Quaternion, Quaternion Quaternion, &Quaternion &Quaternion, &Quaternion }

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn raw() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);

        assert_eq!(1.0, quaternion.w());
        assert_eq!(2.0, quaternion.x());
        assert_eq!(3.0, quaternion.y());
        assert_eq!(4.0, quaternion.z());
    }

    #[test]
    fn normalized() {
        let quaternion = Quaternion::normalized(1.0, 2.0, 3.0, 4.0);

        assert_float_absolute_eq!(0.1826, quaternion.w(), 1e-4);
        assert_float_absolute_eq!(0.3651, quaternion.x(), 1e-4);
        assert_float_absolute_eq!(0.5477, quaternion.y(), 1e-4);
        assert_float_absolute_eq!(0.7303, quaternion.z(), 1e-4);

        assert_float_absolute_eq!(1.0, quaternion.len(), 0.1);
    }

    #[test]
    fn from_euler_angles() {
        let vector = Vector3::new(
            30_f64.to_radians(),
            45_f64.to_radians(),
            90_f64.to_radians(),
        );
        let quaternion = Quaternion::from_euler(&vector);

        assert_float_absolute_eq!(0.7010, quaternion.w(), 1e-4);
        assert_float_absolute_eq!(-0.0922, quaternion.x(), 1e-4);
        assert_float_absolute_eq!(0.4304, quaternion.y(), 1e-4);
        assert_float_absolute_eq!(0.5609, quaternion.z(), 1e-3);
    }

    #[test]
    fn to_euler_angles() {
        let quaternion = Quaternion::normalized(0.7010, -0.0922, 0.4304, 0.5609);
        let vector: Vector3 = quaternion.to_euler();

        assert_float_absolute_eq!(30.0, vector.x().to_degrees(), 1.0);
        assert_float_absolute_eq!(45.0, vector.y().to_degrees(), 1.0);
        assert_float_absolute_eq!(90.0, vector.z().to_degrees(), 1.0);
    }

    #[test]
    fn from_axis_angle() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        let angle = 45_f64.to_radians();

        let actual = Quaternion::from_axis_angle(&vector, angle);

        assert_float_absolute_eq!(0.9239, actual.w(), 1e-4);
        assert_float_absolute_eq!(0.1023, actual.x(), 1e-4);
        assert_float_absolute_eq!(0.2046, actual.y(), 1e-4);
        assert_float_absolute_eq!(0.3068, actual.z(), 1e-4);
    }

    #[test]
    fn to_axis_angle() {
        let quaternion = Quaternion::normalized(0.9239, 0.1023, 0.2046, 0.3068);
        let (vector, angle) = quaternion.to_axis_angle();

        assert_float_absolute_eq!(0.2673, vector.x(), 1e-3);
        assert_float_absolute_eq!(0.5345, vector.y(), 1e-3);
        assert_float_absolute_eq!(0.8018, vector.z(), 1e-3);
        assert_float_absolute_eq!(45.0, angle.to_degrees(), 1e-2);
    }

    #[test]
    fn from_vector() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        let quaternion: Quaternion = vector.into();

        assert_eq!(0.0, quaternion.w());
        assert_eq!(1.0, quaternion.x());
        assert_eq!(2.0, quaternion.y());
        assert_eq!(3.0, quaternion.z());
    }

    #[test]
    fn conjugate() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let conjugate = quaternion.conjugate();

        assert_eq!(1.0, conjugate.w());
        assert_eq!(-2.0, conjugate.x());
        assert_eq!(-3.0, conjugate.y());
        assert_eq!(-4.0, conjugate.z());
    }

    #[test]
    fn conjugate_multiply() {
        let quaternion = Quaternion::normalized(1.0, 2.0, 3.0, 4.0);
        let conjugate = quaternion.conjugate();

        let actual = quaternion * conjugate;

        assert_float_absolute_eq!(1.0, actual.w(), 0.1);
        assert_float_absolute_eq!(0.0, actual.x(), 0.1);
        assert_float_absolute_eq!(0.0, actual.y(), 0.1);
        assert_float_absolute_eq!(0.0, actual.z(), 0.1);
    }

    #[test]
    fn multiply() {
        let q1 = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::raw(5.0, 6.0, 7.0, 8.0);
        let q3 = q1 * q2;

        assert_eq!(-60.0, q3.w());
        assert_eq!(12.0, q3.x());
        assert_eq!(30.0, q3.y());
        assert_eq!(24.0, q3.z());
    }

    #[test]
    fn length() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let len = quaternion.len();

        assert_float_absolute_eq!(5.4772, len, 1e-3);
    }

    #[test]
    fn normalize_regular() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let normalized = quaternion.normalize();

        assert_float_absolute_eq!(0.1826, normalized.w(), 1e-4);
        assert_float_absolute_eq!(0.3651, normalized.x(), 1e-4);
        assert_float_absolute_eq!(0.5477, normalized.y(), 1e-4);
        assert_float_absolute_eq!(0.7303, normalized.z(), 1e-4);

        assert_float_absolute_eq!(1.0, normalized.len(), 0.1);
    }

    #[test]
    fn normalize_identity() {
        let quaternion = Quaternion::IDENTITY;
        let normalized = quaternion.normalize();

        assert_eq!(1.0, normalized.w());
        assert_eq!(0.0, normalized.x());
        assert_eq!(0.0, normalized.y());
        assert_eq!(0.0, normalized.z());

        assert_eq!(1.0, normalized.len());
    }

    #[test]
    #[should_panic]
    fn panic_on_undefined_math() {
        let q = Quaternion {
            w: f64::NAN,
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        };
        let _undefined = q.conjugate();
    }
}
