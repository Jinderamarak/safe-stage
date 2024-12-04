use crate::Axis;
use crate::Quaternion;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Vector 3D
/// Struct representing a 3D vector / 3D point.
///
/// ## Example
/// ```
/// use maths::Vector3;
///
/// let v1 = Vector3::new(1.0, 2.0, 3.0);
/// let v2 = Vector3::new(4.0, 5.0, 6.0);
/// let v3 = v1 * 2.0;
///
/// let v1_v2 = v1 + v2;
/// let v2_v3 = v2 - v3;
///
/// assert_eq!(v1_v2.x(), 5.0);
/// assert_eq!(v2_v3.x(), 2.0);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

//  Eq can be implemented since the constructor checks forbids NaN values
impl Eq for Vector3 {}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3::new(0.0, 0.0, 0.0);

    /// Creates a new vector from raw values.
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        debug_assert!(!x.is_nan() && !y.is_nan() && !z.is_nan());
        Self { x, y, z }
    }

    /// Returns the vector's `x` component.
    #[inline]
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Returns the vector's `y` component.
    #[inline]
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Returns the vector's `z` component.
    #[inline]
    pub const fn z(&self) -> f64 {
        self.z
    }

    /// Returns the vector's component according to the given axis.
    #[inline]
    pub const fn get(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    /// Returns a rotated vector according to the given rotation.
    #[inline]
    pub fn rotate(&self, rotation: &Quaternion) -> Self {
        let rotated = rotation * Quaternion::from(self) * rotation.conjugate();
        rotated.into()
    }

    /// Returns a rotated vector around the given pivot according to the given rotation.
    #[inline]
    pub fn rotate_around(&self, rotation: &Quaternion, pivot: &Vector3) -> Self {
        (*self - pivot).rotate(rotation) + pivot
    }

    /// Returns the dot product of the vector and the `other` vector.
    #[inline]
    pub const fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product of the vector and the `other` vector.
    #[inline]
    pub const fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Returns the length of the vector **squared**
    #[inline]
    pub const fn len2(&self) -> f64 {
        self.dot(self)
    }

    /// Returns the length of the vector.
    #[inline]
    pub fn len(&self) -> f64 {
        self.len2().sqrt()
    }

    /// Returns the normalized vector.
    #[inline]
    pub fn normalize(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            *self / len
        } else {
            *self
        }
    }

    /// Returns the vector with each component clamped to the given min and max values.
    #[inline]
    pub fn clamp(&self, min: &Vector3, max: &Vector3) -> Vector3 {
        Vector3::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }

    /// Takes the minimal components of the two vectors.
    #[inline]
    pub fn minimized(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// Takes the maximal components of the two vectors.
    #[inline]
    pub fn maximized(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Returns the absolute vector.
    #[inline]
    pub fn abs(&self) -> Vector3 {
        Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Returns a linear interpolation between the vector and the `other` vector.
    #[inline]
    pub fn lerp(&self, other: &Vector3, t: f64) -> Vector3 {
        *self + (*other - *self) * t
    }
}

macro_rules! neg_impl {
    ($($t:ty)*) => ($(
        impl Neg for $t {
            type Output = Vector3;

            #[inline]
            fn neg(self) -> Self::Output {
                Vector3::new(-self.x, -self.y, -self.z)
            }
        }
    )*)
}

neg_impl! { Vector3 &Vector3 }

macro_rules! add_impl {
    ($($t1:ty, $t2:ty)*) => ($(
        impl Add<$t2> for $t1 {
            type Output = Vector3;

            #[inline]
            fn add(self, other: $t2) -> Self::Output {
                Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }
    )*)
}

add_impl! { Vector3, Vector3 &Vector3, Vector3 Vector3, &Vector3 &Vector3, &Vector3 }

macro_rules! add_assign_impl {
    ($($t:ty)*) => ($(
        impl AddAssign<$t> for Vector3 {
            #[inline]
            fn add_assign(&mut self, other: $t) {
                *self = *self + other;
            }
        }
    )*)
}

add_assign_impl! { Vector3 &Vector3 }

macro_rules! sub_impl {
    ($($t1:ty, $t2:ty)*) => ($(
        impl Sub<$t2> for $t1 {
            type Output = Vector3;

            #[inline]
            fn sub(self, other: $t2) -> Self::Output {
                Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }
    )*)
}

sub_impl! { Vector3, Vector3 &Vector3, Vector3 Vector3, &Vector3 &Vector3, &Vector3 }

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul<f64> for $t {
            type Output = Vector3;

            #[inline]
            fn mul(self, scalar: f64) -> Vector3 {
                Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
            }
        }
    )*)
}

mul_impl! { Vector3 &Vector3 }

macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div<f64> for $t {
            type Output = Vector3;

            #[inline]
            fn div(self, scalar: f64) -> Vector3 {
                Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
            }
        }
    )*)
}

div_impl! { Vector3 &Vector3 }

macro_rules! from_quaternion_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for Vector3 {
            #[inline]
            fn from(quaternion: $t) -> Self {
                Self::new(quaternion.x(), quaternion.y(), quaternion.z())
            }
        }
    )*)
}

from_quaternion_impl! { &Quaternion Quaternion }

#[cfg(test)]
mod tests {
    use std::f64;

    use super::*;
    use assert_float_eq::*;

    #[test]
    fn create_correct() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, vector.x());
        assert_eq!(2.0, vector.y());
        assert_eq!(3.0, vector.z());
    }

    #[test]
    fn add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let r = v1 + v2;

        assert_eq!(5.0, r.x());
        assert_eq!(7.0, r.y());
        assert_eq!(9.0, r.z());

        let mut r = v1;
        r += v2;

        assert_eq!(5.0, r.x());
        assert_eq!(7.0, r.y());
        assert_eq!(9.0, r.z());
    }

    #[test]
    fn subtract() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let r = v1 - v2;

        assert_eq!(-3.0, r.x());
        assert_eq!(-3.0, r.y());
        assert_eq!(-3.0, r.z());
    }

    #[test]
    fn multiply() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let r = v1 * 2.0;

        assert_eq!(2.0, r.x());
        assert_eq!(4.0, r.y());
        assert_eq!(6.0, r.z());
    }

    #[test]
    fn divide() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let r = v1 / 2.0;

        assert_eq!(0.5, r.x());
        assert_eq!(1.0, r.y());
        assert_eq!(1.5, r.z());
    }

    #[test]
    fn rotate_90() {
        let vector = Vector3::new(3.0, 0.0, 0.0);
        let rotation = Quaternion::from_euler(&Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
        ));
        let rotated = vector.rotate(&rotation);

        assert_float_absolute_eq!(0.0, rotated.x());
        assert_float_absolute_eq!(3.0, rotated.y());
        assert_float_absolute_eq!(0.0, rotated.z());
    }

    #[test]
    fn rotate_45() {
        let vector = Vector3::new(1.0, 1.0, 1.0);
        let rotation = Quaternion::from_euler(&Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            45.0_f64.to_radians(),
        ));
        let rotated = vector.rotate(&rotation);

        assert_float_absolute_eq!(0.0, rotated.x());
        assert_float_absolute_eq!(f64::consts::SQRT_2, rotated.y(), 1e-4);
        assert_float_absolute_eq!(1.0, rotated.z());
    }

    #[test]
    fn rotate_x() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        let rotation = Quaternion::from_euler(&Vector3::new(
            90.0_f64.to_radians(),
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
        ));
        let rotated = vector.rotate(&rotation);

        assert_float_absolute_eq!(1.0, rotated.x());
        assert_float_absolute_eq!(-3.0, rotated.y());
        assert_float_absolute_eq!(2.0, rotated.z());
    }

    #[test]
    fn rotate_inverse() {
        let vector = Vector3::new(4.0, 0.0, 0.0);
        let rotation = Quaternion::from_euler(&Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
        ));
        let rotated = vector.rotate(&rotation.conjugate());

        assert_float_absolute_eq!(0.0, rotated.x());
        assert_float_absolute_eq!(-4.0, rotated.y());
        assert_float_absolute_eq!(0.0, rotated.z());
    }

    #[test]
    fn rotate_around_z() {
        let vector = Vector3::new(7.0, 1.0, 0.0);
        let pivot = Vector3::new(1.0, 1.0, 0.0);
        let rotation = Quaternion::from_euler(&Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
        ));
        let rotated = vector.rotate_around(&rotation, &pivot);

        assert_float_absolute_eq!(1.0, rotated.x());
        assert_float_absolute_eq!(7.0, rotated.y());
        assert_float_absolute_eq!(0.0, rotated.z());
    }

    #[test]
    fn rotate_around_y() {
        let vector = Vector3::new(2.0, 2.0, 1.0);
        let pivot = Vector3::new(1.0, 1.0, 1.0);
        let rotation = Quaternion::from_euler(&Vector3::new(
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
            0.0_f64.to_radians(),
        ));
        let rotated = vector.rotate_around(&rotation, &pivot);

        assert_float_absolute_eq!(1.0, rotated.x());
        assert_float_absolute_eq!(2.0, rotated.y());
        assert_float_absolute_eq!(0.0, rotated.z());
    }

    #[test]
    fn dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        let expected = 32.0;
        let actual = v1.dot(&v2);
        let commutative = v2.dot(&v1);

        assert_float_absolute_eq!(expected, actual);
        assert_float_absolute_eq!(expected, commutative);
    }

    #[test]
    fn cross_product() {
        let v1 = Vector3::new(3.0, 0.0, 2.0);
        let v2 = Vector3::new(-1.0, 4.0, 2.0);

        let expected = Vector3::new(-8.0, -8.0, 12.0);
        let actual = v1.cross(&v2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn length_squared() {
        let vector = Vector3::new(1.0, 2.0, 3.0);

        let expected = 14.0;
        let actual = vector.len2();

        assert_float_absolute_eq!(expected, actual);
    }

    #[test]
    fn length() {
        let vector = Vector3::new(1.0, 2.0, 3.0);

        let expected = 14.0_f64.sqrt();
        let actual = vector.len();

        assert_float_absolute_eq!(expected, actual);
    }

    #[test]
    fn normalize() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        let unit = vector.normalize();

        let expected = 1.0;
        let actual = unit.len();

        assert_float_absolute_eq!(expected, actual);
    }

    #[test]
    fn normalize_zero_length() {
        let vector = Vector3::new(0.0, 0.0, 0.0);
        let unit = vector.normalize();

        assert_eq!(vector, unit);
    }

    #[test]
    fn from_quaternion() {
        let quaternion = Quaternion::raw(1.0, 2.0, 3.0, 4.0);
        let vector: Vector3 = quaternion.into();

        assert_eq!(2.0, vector.x());
        assert_eq!(3.0, vector.y());
        assert_eq!(4.0, vector.z());
    }

    #[test]
    fn clamp() {
        let vector = Vector3::new(2.0, 1.0, 0.0);
        let min = Vector3::new(-1.0, 0.0, 1.0);
        let max = Vector3::new(1.0, 1.0, 2.0);
        let clamped = vector.clamp(&min, &max);

        assert_eq!(1.0, clamped.x());
        assert_eq!(1.0, clamped.y());
        assert_eq!(1.0, clamped.z());
    }

    #[test]
    fn minimized() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 1.0, 6.0);
        let min = v1.minimized(&v2);

        assert_eq!(1.0, min.x());
        assert_eq!(1.0, min.y());
        assert_eq!(3.0, min.z());
    }

    #[test]
    fn maximized() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 1.0, 6.0);
        let max = v1.maximized(&v2);

        assert_eq!(4.0, max.x());
        assert_eq!(2.0, max.y());
        assert_eq!(6.0, max.z());
    }

    #[test]
    fn abs() {
        let vector = Vector3::new(-1.0, -2.0, -3.0);
        let abs = vector.abs();

        assert_eq!(1.0, abs.x());
        assert_eq!(2.0, abs.y());
        assert_eq!(3.0, abs.z());
    }

    #[test]
    fn lerp() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let lerp = v1.lerp(&v2, 0.5);

        assert_eq!(2.5, lerp.x());
        assert_eq!(3.5, lerp.y());
        assert_eq!(4.5, lerp.z());
    }

    #[test]
    #[should_panic]
    fn panic_on_undefined_math() {
        let v = Vector3::new(0.0, 0.0, 0.0);
        let _undefined = v / 0.0;
    }
}
