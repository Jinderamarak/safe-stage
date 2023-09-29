use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
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
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3_correct() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(vector.x(), 1.0);
        assert_eq!(vector.y(), 2.0);
        assert_eq!(vector.z(), 3.0);
    }
}
