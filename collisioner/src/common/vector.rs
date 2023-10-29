use crate::common::Axis;
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn create_correct() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(vector.x(), 1.0);
        assert_eq!(vector.y(), 2.0);
        assert_eq!(vector.z(), 3.0);
    }

    #[test]
    fn add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let r = v1 + v2;

        assert_eq!(r.x(), 5.0);
        assert_eq!(r.y(), 7.0);
        assert_eq!(r.z(), 9.0);
    }
}
