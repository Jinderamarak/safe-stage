use crate::common::{Axis, Quaternion};
use std::ops::{Add, Div, Mul, Sub};

/// # Vector 3
/// Struct representing a 3D vector / 3D point.
///
/// ## Example
/// ```
/// use collisioner::common::Vector3;
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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

    pub fn get(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn rotate(&self, rotation: Quaternion) -> Self {
        let rotated = rotation * self.into() * rotation.conjugate();
        rotated.into()
    }

    pub fn rotate_around(&self, pivot: Vector3, rotation: Quaternion) -> Self {
        let rotated = rotation * (*self - pivot).into() * rotation.conjugate();
        pivot + rotated.into()
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn len(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.len();
        if len > 0.0 {
            *self / len
        } else {
            *self
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl From<&Quaternion> for Vector3 {
    fn from(quaternion: &Quaternion) -> Self {
        Self::new(quaternion.x(), quaternion.y(), quaternion.z())
    }
}

impl From<Quaternion> for Vector3 {
    fn from(quaternion: Quaternion) -> Self {
        Self::from(&quaternion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    use pretty_assertions::assert_eq;

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
    fn rotate() {
        let vector = Vector3::new(1.0, 0.0, 0.0);
        let rotation = Quaternion::from_euler(Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
        ));
        let rotated = vector.rotate(rotation);

        assert_float_absolute_eq!(0.0, rotated.x());
        assert_float_absolute_eq!(1.0, rotated.y());
        assert_float_absolute_eq!(0.0, rotated.z());
    }

    #[test]
    fn rotate_around() {
        let vector = Vector3::new(1.0, 0.0, 0.0);
        let pivot = Vector3::new(1.0, 1.0, 0.0);
        let rotation = Quaternion::from_euler(Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            90.0_f64.to_radians(),
        ));
        let rotated = vector.rotate_around(pivot, rotation);

        assert_float_absolute_eq!(2.0, rotated.x());
        assert_float_absolute_eq!(1.0, rotated.y());
        assert_float_absolute_eq!(0.0, rotated.z());
    }

    #[test]
    fn dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        let expected = 32.0;
        let actual = v1.dot(v2);
        let commutative = v2.dot(v1);

        assert_float_absolute_eq!(expected, actual);
        assert_float_absolute_eq!(expected, commutative);
    }

    #[test]
    fn cross_product() {
        let v1 = Vector3::new(3.0, 0.0, 2.0);
        let v2 = Vector3::new(-1.0, 4.0, 2.0);

        let expected = Vector3::new(-8.0, -8.0, 12.0);
        let actual = v1.cross(v2);

        assert_eq!(expected, actual);
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
    fn from_quaternion() {
        let quaternion = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let vector: Vector3 = quaternion.into();

        assert_eq!(1.0, vector.x());
        assert_eq!(2.0, vector.y());
        assert_eq!(3.0, vector.z());
    }
}
