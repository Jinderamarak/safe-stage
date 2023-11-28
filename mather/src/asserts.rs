use crate::{Quaternion, Vector3};

pub fn assert_vectors(expected: Vector3, actual: Vector3) {
    assert_vectors_close(expected, actual, f64::EPSILON);
}

pub fn assert_vectors_close(expected: Vector3, actual: Vector3, eps: f64) {
    assert!(
        (expected.x() - actual.x()).abs() < eps,
        "Vector X expected: {}, actual: {}",
        expected.x(),
        actual.x()
    );
    assert!(
        (expected.y() - actual.y()).abs() < eps,
        "Vector Y expected: {}, actual: {}",
        expected.y(),
        actual.y()
    );
    assert!(
        (expected.z() - actual.z()).abs() < eps,
        "Vector Z expected: {}, actual: {}",
        expected.z(),
        actual.z()
    );
}

pub fn assert_quaternion(expected: Quaternion, actual: Quaternion) {
    assert_quaternion_close(expected, actual, f64::EPSILON);
}

pub fn assert_quaternion_close(expected: Quaternion, actual: Quaternion, eps: f64) {
    assert!(
        (expected.x() - actual.x()).abs() < eps,
        "Quaternion X expected: {}, actual: {}",
        expected.x(),
        actual.x()
    );
    assert!(
        (expected.y() - actual.y()).abs() < eps,
        "Quaternion Y expected: {}, actual: {}",
        expected.y(),
        actual.y()
    );
    assert!(
        (expected.z() - actual.z()).abs() < eps,
        "Quaternion Z expected: {}, actual: {}",
        expected.z(),
        actual.z()
    );
    assert!(
        (expected.w() - actual.w()).abs() < eps,
        "Quaternion W expected: {}, actual: {}",
        expected.w(),
        actual.w()
    );
}
