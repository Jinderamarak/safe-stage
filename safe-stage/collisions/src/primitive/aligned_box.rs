use crate::collides_group_impl;
use crate::common::{
    Bounded, Collides, Projectable, Rotation, Transformation, Translation, Treeable,
};
use crate::primitive::{OrientedBoxCollider, PointCollider, SphereCollider, TriangleCollider};
use itertools::Itertools;
use maths::{Quaternion, Vector3};

/// # Axis Aligned Box Collider
/// Collision primitive for box that is aligned with the x,y,z axes.
///
/// ## Example
/// ```
/// use collisions::primitive::AlignedBoxCollider;
/// use collisions::common::Collides;
/// use maths::Vector3;
///
/// let box1 = AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
/// let box2 = AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0));
/// assert!(box1.collides_with(&box2));
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct AlignedBoxCollider {
    min: Vector3,
    max: Vector3,
}

impl AlignedBoxCollider {
    /// Creates a new `AlignedBoxCollider` with the given `center` and `size`.
    pub fn new(center: Vector3, size: Vector3) -> Self {
        let size = size.abs();
        Self {
            min: center - size / 2.0,
            max: center + size / 2.0,
        }
    }

    #[inline]
    pub const fn from_min_max(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }

    /// Returns the size of the box.
    #[inline]
    pub fn size(&self) -> Vector3 {
        self.max - self.min
    }

    #[inline]
    fn corners(&self) -> [Vector3; 8] {
        [
            Vector3::new(self.min.x(), self.min.y(), self.min.z()),
            Vector3::new(self.max.x(), self.min.y(), self.min.z()),
            Vector3::new(self.min.x(), self.max.y(), self.min.z()),
            Vector3::new(self.max.x(), self.max.y(), self.min.z()),
            Vector3::new(self.min.x(), self.min.y(), self.max.z()),
            Vector3::new(self.max.x(), self.min.y(), self.max.z()),
            Vector3::new(self.min.x(), self.max.y(), self.max.z()),
            Vector3::new(self.max.x(), self.max.y(), self.max.z()),
        ]
    }
}

impl Treeable for AlignedBoxCollider {
    fn bound_children(&self, other: &Self) -> Self {
        let min = self.min.minimized(&other.min);
        let max = self.max.maximized(&other.max);
        Self::from_min_max(min, max)
    }

    fn bound_triangle(triangle: &TriangleCollider) -> Self {
        let (a, b, c) = triangle.points();
        let min = Vector3::new(
            a.x().min(b.x()).min(c.x()),
            a.y().min(b.y()).min(c.y()),
            a.z().min(b.z()).min(c.z()),
        );
        let max = Vector3::new(
            a.x().max(b.x()).max(c.x()),
            a.y().max(b.y()).max(c.y()),
            a.z().max(b.z()).max(c.z()),
        );
        Self::from_min_max(min, max)
    }
}

impl Bounded for AlignedBoxCollider {
    #[inline]
    fn min(&self) -> Vector3 {
        self.min
    }

    #[inline]
    fn max(&self) -> Vector3 {
        self.max
    }
}

impl Projectable for AlignedBoxCollider {
    fn project(&self, axis: &Vector3) -> (f64, f64) {
        self.corners()
            .iter()
            .map(|corner| axis.dot(corner))
            .minmax()
            .into_option()
            .unwrap()
    }
}

impl Rotation<OrientedBoxCollider> for AlignedBoxCollider {
    fn rotate(&self, rotation: &Quaternion) -> OrientedBoxCollider {
        OrientedBoxCollider::from(self).rotate(rotation)
    }

    fn rotate_around(&self, rotation: &Quaternion, point: &Vector3) -> OrientedBoxCollider {
        OrientedBoxCollider::from(self).rotate_around(rotation, point)
    }
}

impl Translation for AlignedBoxCollider {
    fn translate(&self, translation: &Vector3) -> Self {
        Self::new(self.center() + translation, self.size())
    }
}

impl Transformation<OrientedBoxCollider> for AlignedBoxCollider {
    fn transform(
        &self,
        rotation: &Quaternion,
        pivot: &Vector3,
        translation: &Vector3,
    ) -> OrientedBoxCollider {
        OrientedBoxCollider::from(self).transform(rotation, pivot, translation)
    }
}

impl Collides<Self> for AlignedBoxCollider {
    fn collides_with(&self, other: &Self) -> bool {
        let self_min = self.min;
        let self_max = self.max;
        let other_min = other.min;
        let other_max = other.max;

        self_min.x() <= other_max.x()
            && self_max.x() >= other_min.x()
            && self_min.y() <= other_max.y()
            && self_max.y() >= other_min.y()
            && self_min.z() <= other_max.z()
            && self_max.z() >= other_min.z()
    }
}

impl Collides<PointCollider> for AlignedBoxCollider {
    fn collides_with(&self, point: &PointCollider) -> bool {
        let self_min = self.min();
        let self_max = self.max();
        let position = point.position();

        self_min.x() <= position.x()
            && self_max.x() >= position.x()
            && self_min.y() <= position.y()
            && self_max.y() >= position.y()
            && self_min.z() <= position.z()
            && self_max.z() >= position.z()
    }
}

impl Collides<SphereCollider> for AlignedBoxCollider {
    fn collides_with(&self, other: &SphereCollider) -> bool {
        let min = self.min();
        let max = self.max();
        let center = other.center();

        let distance_squared = (min.x().max(center.x()).min(max.x()) - center.x()).powi(2)
            + (min.y().max(center.y()).min(max.y()) - center.y()).powi(2)
            + (min.z().max(center.z()).min(max.z()) - center.z()).powi(2);

        distance_squared < other.radius().powi(2)
    }
}

impl Collides<OrientedBoxCollider> for AlignedBoxCollider {
    fn collides_with(&self, other: &OrientedBoxCollider) -> bool {
        other.collides_with(self)
    }
}

collides_group_impl!(
    AlignedBoxCollider, PointCollider
    AlignedBoxCollider, SphereCollider
    AlignedBoxCollider, AlignedBoxCollider
    AlignedBoxCollider, OrientedBoxCollider
);

#[cfg(test)]
mod tests {
    use super::*;
    use maths::asserts::{assert_quaternion, assert_vectors};

    #[test]
    fn bounds_regular() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 2.0, 4.0), Vector3::new(3.0, 2.0, 1.0));

        assert_vectors(Vector3::new(-0.5, 1.0, 3.5), aabb.min());
        assert_vectors(Vector3::new(2.5, 3.0, 4.5), aabb.max());
    }

    #[test]
    fn projection() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 2.0, 4.0), Vector3::new(3.0, 2.0, 1.0));

        assert_eq!((-0.5, 2.5), aabb.project(&Vector3::new(1.0, 0.0, 0.0)));
        assert_eq!((1.0, 3.0), aabb.project(&Vector3::new(0.0, 1.0, 0.0)));
        assert_eq!((3.5, 4.5), aabb.project(&Vector3::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn rotation() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 4.0));
        let rotation = Quaternion::from_euler(&Vector3::new(0.0, 90.0_f64.to_radians(), 0.0));

        let rotated = aabb.rotate(&rotation);

        assert_vectors(Vector3::new(1.0, 1.0, 1.0), rotated.center());
        assert_vectors(Vector3::new(-1.0, 0.0, 0.0), rotated.min());
        assert_vectors(Vector3::new(3.0, 2.0, 2.0), rotated.max());
    }

    #[test]
    fn pivot_rotation() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(2.0, 2.0, 1.0), Vector3::new(2.0, 2.0, 4.0));
        let rotation = Quaternion::from_euler(&Vector3::new(0.0, 90.0_f64.to_radians(), 0.0));

        let rotated = aabb.rotate_around(&rotation, &Vector3::new(1.0, 1.0, 1.0));

        assert_vectors(Vector3::new(1.0, 2.0, 0.0), rotated.center());
        assert_vectors(Vector3::new(-1.0, 1.0, -1.0), rotated.min());
        assert_vectors(Vector3::new(3.0, 3.0, 1.0), rotated.max());
    }

    #[test]
    fn translation() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 4.0));
        let translation = Vector3::new(1.0, 2.0, 3.0);

        let translated = aabb.translate(&translation);

        assert_vectors(Vector3::new(2.0, 3.0, 4.0), translated.center());
    }

    #[test]
    fn transformation() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0));
        let rotation = Quaternion::from_euler(&Vector3::new(0.0, 90.0_f64.to_radians(), 0.0));
        let pivot = Vector3::new(0.0, 0.0, 0.0);
        let translation = Vector3::new(-1.0, 1.0, 1.0);

        let transformed = aabb.transform(&rotation, &pivot, &translation);

        assert_vectors(Vector3::new(0.0, 2.0, 0.0), transformed.center());
        assert_quaternion(rotation, transformed.rotation());
    }

    #[test]
    fn aabb_corner_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_edge_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_face_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_overlap_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_inside_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_outside_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(2.0, 2.0, 2.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!aabb.collides_with(&other_aabb));
    }

    #[test]
    fn aabb_close_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let other_aabb =
            AlignedBoxCollider::new(Vector3::new(2.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!aabb.collides_with(&other_aabb));
    }

    #[test]
    fn point_corner_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));

        assert!(aabb.collides_with(&point));
    }

    #[test]
    fn point_edge_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 0.0));

        assert!(aabb.collides_with(&point));
    }

    #[test]
    fn point_face_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(1.0, 0.0, 0.0));

        assert!(aabb.collides_with(&point));
    }

    #[test]
    fn point_inside_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(0.0, 0.0, 0.0));

        assert!(aabb.collides_with(&point));
    }

    #[test]
    fn point_outside_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let point = PointCollider::new(Vector3::new(2.0, 2.0, 2.0));

        assert!(!aabb.collides_with(&point));
    }

    #[test]
    fn point_close_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let point = PointCollider::new(Vector3::new(3.0, 1.0, 1.0));

        assert!(!aabb.collides_with(&point));
    }

    #[test]
    fn sphere_corner_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 1.733);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_corner_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 1.73);

        assert!(!aabb.collides_with(&sphere));
        assert!(!sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_edge_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.70, 1.70, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_edge_dont_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.71, 1.71, 0.0), 1.0);

        assert!(!aabb.collides_with(&sphere));
        assert!(!sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_face_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(1.0, 0.0, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }

    #[test]
    fn sphere_inside_collide() {
        let aabb =
            AlignedBoxCollider::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(2.0, 2.0, 2.0));
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);

        assert!(aabb.collides_with(&sphere));
        assert!(sphere.collides_with(&aabb));
    }
}
