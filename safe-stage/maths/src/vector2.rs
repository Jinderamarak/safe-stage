use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Vector 2D
/// Struct representing a 2D vector / 2D point.
///
/// ## Example
/// ```
/// use maths::Vector2;
///
/// let v1 = Vector2::new(1.0, 2.0);
/// let v2 = Vector2::new(4.0, 5.0);
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
pub struct Vector2 {
    x: f64,
    y: f64,
}

//  Eq can be implemented since the constructor checks forbids NaN values
impl Eq for Vector2 {}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2::new(0.0, 0.0);

    /// Creates a new vector from raw values.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        debug_assert!(!x.is_nan() && !y.is_nan());
        Self { x, y }
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

    /// Returns the dot product of the vector and the `other` vector.
    #[inline]
    pub const fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Returns 2D cross product.
    /// Represents signed magnitude of the vector in 3D space.
    #[inline]
    pub const fn cross(&self, other: &Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Returns the length of the vector.
    #[inline]
    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
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

    /// Returns the absolute vector.
    #[inline]
    pub fn abs(&self) -> Vector2 {
        Vector2::new(self.x.abs(), self.y.abs())
    }
}

macro_rules! neg_impl {
    ($($t:ty)*) => ($(
        impl Neg for $t {
            type Output = Vector2;

            #[inline]
            fn neg(self) -> Self::Output {
                Vector2::new(-self.x, -self.y)
            }
        }
    )*)
}

neg_impl! { Vector2 &Vector2 }

macro_rules! add_impl {
    ($($t1:ty, $t2:ty)*) => ($(
        impl Add<$t2> for $t1 {
            type Output = Vector2;

            #[inline]
            fn add(self, other: $t2) -> Self::Output {
                Vector2::new(self.x + other.x, self.y + other.y)
            }
        }
    )*)
}

add_impl! { Vector2, Vector2 &Vector2, Vector2 Vector2, &Vector2 &Vector2, &Vector2 }

macro_rules! add_assign_impl {
    ($($t:ty)*) => ($(
        impl AddAssign<$t> for Vector2 {
            #[inline]
            fn add_assign(&mut self, other: $t) {
                *self = *self + other;
            }
        }
    )*)
}

add_assign_impl! { Vector2 &Vector2 }

macro_rules! sub_impl {
    ($($t1:ty, $t2:ty)*) => ($(
        impl Sub<$t2> for $t1 {
            type Output = Vector2;

            #[inline]
            fn sub(self, other: $t2) -> Self::Output {
                Vector2::new(self.x - other.x, self.y - other.y)
            }
        }
    )*)
}

sub_impl! { Vector2, Vector2 &Vector2, Vector2 Vector2, &Vector2 &Vector2, &Vector2 }

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul<f64> for $t {
            type Output = Vector2;

            #[inline]
            fn mul(self, scalar: f64) -> Self::Output {
                Vector2::new(self.x * scalar, self.y * scalar)
            }
        }
    )*)
}

mul_impl! { Vector2 &Vector2 }

macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div<f64> for $t {
            type Output = Vector2;

            #[inline]
            fn div(self, scalar: f64) -> Self::Output {
                Vector2::new(self.x / scalar, self.y / scalar)
            }
        }
    )*)
}

div_impl! { Vector2 &Vector2 }

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn add() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(4.0, 5.0);
        let r = v1 + v2;

        assert_eq!(5.0, r.x());
        assert_eq!(7.0, r.y());

        let mut r = v1;
        r += v2;

        assert_eq!(5.0, r.x());
        assert_eq!(7.0, r.y());
    }

    #[test]
    fn subtract() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(4.0, 5.0);
        let r = v1 - v2;

        assert_eq!(-3.0, r.x());
        assert_eq!(-3.0, r.y());
    }

    #[test]
    fn multiply() {
        let v1 = Vector2::new(1.0, 2.0);
        let r = v1 * 2.0;

        assert_eq!(2.0, r.x());
        assert_eq!(4.0, r.y());
    }

    #[test]
    fn divide() {
        let v1 = Vector2::new(1.0, 2.0);
        let r = v1 / 2.0;

        assert_eq!(0.5, r.x());
        assert_eq!(1.0, r.y());
    }

    #[test]
    fn dot_product() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(4.0, 5.0);

        let expected = 14.0;
        let actual = v1.dot(&v2);
        let commutative = v2.dot(&v1);

        assert_float_absolute_eq!(expected, actual);
        assert_float_absolute_eq!(expected, commutative);
    }

    #[test]
    fn cross_product() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(4.0, 5.0);

        let expected = -3.0;
        let actual = v1.cross(&v2);
        let commutative = v2.cross(&v1);

        assert_float_absolute_eq!(expected, actual);
        assert_float_absolute_eq!(-expected, commutative);
    }

    #[test]
    fn length() {
        let vector = Vector2::new(3.0, 4.0);

        let expected = 5.0;
        let actual = vector.len();

        assert_float_absolute_eq!(expected, actual);
    }

    #[test]
    fn normalize() {
        let vector = Vector2::new(1.0, 2.0);
        let unit = vector.normalize();

        let expected = 1.0;
        let actual = unit.len();

        assert_float_absolute_eq!(expected, actual);
    }

    #[test]
    fn normalize_zero_length() {
        let vector = Vector2::new(0.0, 0.0);
        let unit = vector.normalize();

        assert_eq!(vector, unit);
    }

    #[test]
    fn absolute() {
        let vector = Vector2::new(-1.0, -2.0);
        let abs = vector.abs();

        assert_eq!(1.0, abs.x());
        assert_eq!(2.0, abs.y());
    }

    #[test]
    #[should_panic]
    fn panic_on_undefined_math() {
        let v = Vector2::new(0.0, 0.0);
        let _undefined = v / 0.0;
    }
}
