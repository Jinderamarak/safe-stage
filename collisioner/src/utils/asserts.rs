use crate::math::{Quaternion, Vector3};
use assert_float_eq::*;

pub fn assert_vector(expected: Vector3, actual: Vector3) {
    assert_float_absolute_eq!(expected.x(), actual.x());
    assert_float_absolute_eq!(expected.y(), actual.y());
    assert_float_absolute_eq!(expected.z(), actual.z());
}

pub fn assert_quaternion(expected: Quaternion, actual: Quaternion) {
    assert_float_absolute_eq!(expected.x(), actual.x());
    assert_float_absolute_eq!(expected.y(), actual.y());
    assert_float_absolute_eq!(expected.z(), actual.z());
    assert_float_absolute_eq!(expected.w(), actual.w());
}
