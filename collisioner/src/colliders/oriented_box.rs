use crate::colliders::{
    AlignedBoxCollider, Bounded, Collides, PointCollider, Projectable, Rotation, SphereCollider,
};
use crate::common::{Quaternion, Vector3};
use itertools::Itertools;

/// # Oriented Bounding Box Collider
/// Advanced primitive for collision detection of boxes with rotation.
///
/// ## Example
/// ```
/// use assert_float_eq::*;
/// use collisioner::colliders::OrientedBoxCollider;
/// use collisioner::common::{Quaternion, Vector3};
///
/// let position = Vector3::new(0.0, 0.0, 0.0);
/// let size = Vector3::new(2.0, 2.0, 2.0);
/// let rotation = Vector3::new(0.0, 0.0, 90.0_f64.to_radians());
///
/// let obb = OrientedBoxCollider::new(position, size, Quaternion::from_euler(rotation));
///
/// assert_eq!(Vector3::new(0.0, 0.0, 0.0), obb.position());
/// assert_eq!(Vector3::new(2.0, 2.0, 2.0), obb.size());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OrientedBoxCollider {
    position: Vector3,
    size: Vector3,
    rotation: Quaternion,
}

impl OrientedBoxCollider {
    pub fn new(position: Vector3, size: Vector3, rotation: Quaternion) -> Self {
        Self {
            position,
            size: Vector3::new(size.x().abs(), size.y().abs(), size.z().abs()),
            rotation,
        }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn size(&self) -> Vector3 {
        self.size
    }

    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }

    fn corners(&self) -> [Vector3; 8] {
        let half_size = self.size() / 2.0;
        let negative_pos = self.position - half_size;
        let positive_pos = self.position + half_size;

        let corners = [
            Vector3::new(negative_pos.x(), negative_pos.y(), negative_pos.z()),
            Vector3::new(positive_pos.x(), negative_pos.y(), negative_pos.z()),
            Vector3::new(negative_pos.x(), positive_pos.y(), negative_pos.z()),
            Vector3::new(positive_pos.x(), positive_pos.y(), negative_pos.z()),
            Vector3::new(negative_pos.x(), negative_pos.y(), positive_pos.z()),
            Vector3::new(positive_pos.x(), negative_pos.y(), positive_pos.z()),
            Vector3::new(negative_pos.x(), positive_pos.y(), positive_pos.z()),
            Vector3::new(positive_pos.x(), positive_pos.y(), positive_pos.z()),
        ];

        corners
            .into_iter()
            .map(|c| c.rotate_around(self.rotation, self.position))
            .collect_vec()
            .try_into()
            .unwrap()
    }

    fn separating_axes(&self) -> (Vector3, Vector3, Vector3) {
        (
            Vector3::new(1.0, 0.0, 0.0).rotate(self.rotation),
            Vector3::new(0.0, 1.0, 0.0).rotate(self.rotation),
            Vector3::new(0.0, 0.0, 1.0).rotate(self.rotation),
        )
    }
}

impl Bounded for OrientedBoxCollider {
    fn min(&self) -> Vector3 {
        self.corners().iter().fold(
            Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            |min, corner| {
                Vector3::new(
                    min.x().min(corner.x()),
                    min.y().min(corner.y()),
                    min.z().min(corner.z()),
                )
            },
        )
    }

    fn max(&self) -> Vector3 {
        self.corners().iter().fold(
            Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            |max, corner| {
                Vector3::new(
                    max.x().max(corner.x()),
                    max.y().max(corner.y()),
                    max.z().max(corner.z()),
                )
            },
        )
    }
}

impl Projectable for OrientedBoxCollider {
    fn project(&self, axis: Vector3) -> (f64, f64) {
        self.corners()
            .iter()
            .map(|corner| axis.dot(*corner))
            .minmax()
            .into_option()
            .unwrap()
    }
}

impl Rotation for OrientedBoxCollider {
    fn rotate(&self, rotation: Quaternion) -> Self {
        self.rotate_around(rotation, self.position)
    }

    fn rotate_around(&self, rotation: Quaternion, pivot: Vector3) -> Self {
        let new_position = self.position.rotate_around(rotation, pivot);
        let new_rotation = self.rotation * rotation;

        Self::new(new_position, self.size, new_rotation)
    }
}

impl From<&AlignedBoxCollider> for OrientedBoxCollider {
    fn from(value: &AlignedBoxCollider) -> Self {
        OrientedBoxCollider::new(
            value.position(),
            value.size(),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        )
    }
}

impl Collides<Self> for OrientedBoxCollider {
    fn collides_with(&self, other: &OrientedBoxCollider) -> bool {
        let (ax, ay, az) = self.separating_axes();
        let (bx, by, bz) = other.separating_axes();

        let axes = [
            ax,
            ay,
            az,
            bx,
            by,
            bz,
            ax.cross(bx),
            ax.cross(by),
            ax.cross(bz),
            ay.cross(bx),
            ay.cross(by),
            ay.cross(bz),
            az.cross(bx),
            az.cross(by),
            az.cross(bz),
        ];

        axes.iter()
            .map(Vector3::normalize)
            .all(|axis| self.intersects(other, axis))
    }
}

impl Collides<PointCollider> for OrientedBoxCollider {
    fn collides_with(&self, other: &PointCollider) -> bool {
        let halfs = self.size / 2.0;
        let min = self.position - halfs;
        let max = self.position + halfs;

        let inverse_point = other.position().rotate(self.rotation.conjugate());

        inverse_point >= min && inverse_point <= max
    }
}

impl Collides<SphereCollider> for OrientedBoxCollider {
    fn collides_with(&self, other: &SphereCollider) -> bool {
        let halfs = self.size / 2.0;
        let min = self.position - halfs;
        let max = self.position + halfs;

        let inverse_center = other.position().rotate(self.rotation.conjugate());
        let clamped = inverse_center.clamp(&min, &max);

        let closest = clamped.rotate(self.rotation);
        let distance = (closest - other.position()).len();

        distance <= other.radius()
    }
}

impl Collides<AlignedBoxCollider> for OrientedBoxCollider {
    fn collides_with(&self, other: &AlignedBoxCollider) -> bool {
        let (ax, ay, az) = self.separating_axes();

        let bx = Vector3::new(1.0, 0.0, 0.0);
        let by = Vector3::new(0.0, 1.0, 0.0);
        let bz = Vector3::new(0.0, 0.0, 1.0);

        let axes = [
            ax,
            ay,
            az,
            bx,
            by,
            bz,
            ax.cross(bx),
            ax.cross(by),
            ax.cross(bz),
            ay.cross(bx),
            ay.cross(by),
            ay.cross(bz),
            az.cross(bx),
            az.cross(by),
            az.cross(bz),
        ];

        axes.iter()
            .map(Vector3::normalize)
            .all(|axis| self.intersects(other, axis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::asserts::*;

    #[test]
    fn pivot_rotation() {
        let collider = OrientedBoxCollider::new(
            Vector3::new(5.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                45.0_f64.to_radians(),
            )),
        );

        let rotated = collider.rotate_around(
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
            Vector3::new(0.0, 0.0, 0.0),
        );

        let position = Vector3::new(0.0, 5.0, 0.0);
        let rotation = Quaternion::from_euler(Vector3::new(
            0.0_f64.to_radians(),
            0.0_f64.to_radians(),
            135.0_f64.to_radians(),
        ));

        assert_vector(position, rotated.position());
        assert_quaternion(rotation, rotated.rotation());
    }

    #[test]
    fn corners() {
        let collider = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 4.0, 6.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );

        let actual_corners = collider.corners();
        let expected_corners = [
            Vector3::new(-2.0, 1.0, 3.0),
            Vector3::new(2.0, 1.0, 3.0),
            Vector3::new(2.0, -1.0, 3.0),
            Vector3::new(-2.0, -1.0, 3.0),
            Vector3::new(-2.0, 1.0, -3.0),
            Vector3::new(2.0, 1.0, -3.0),
            Vector3::new(2.0, -1.0, -3.0),
            Vector3::new(-2.0, -1.0, -3.0),
        ];

        for expected in &expected_corners {
            let mut found = false;
            for actual in &actual_corners {
                if (expected.x() - actual.x()).abs() <= f64::EPSILON * 4.0
                    && (expected.y() - actual.y()).abs() <= f64::EPSILON * 4.0
                    && (expected.z() - actual.z()).abs() <= f64::EPSILON * 4.0
                {
                    found = true;
                    break;
                }
            }

            assert!(found, "Missing corner {:?}", expected);
        }
    }

    #[test]
    fn obb_corner_corner_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );
        let other_obb = OrientedBoxCollider::new(
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );

        assert!(obb.collides_with(&other_obb));
        assert!(other_obb.collides_with(&obb));
    }

    #[test]
    fn obb_edge_edge_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );
        let other_obb = OrientedBoxCollider::new(
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );

        assert!(obb.collides_with(&other_obb));
        assert!(other_obb.collides_with(&obb));
    }

    #[test]
    fn obb_face_face_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );
        let other_obb = OrientedBoxCollider::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );

        assert!(obb.collides_with(&other_obb));
        assert!(other_obb.collides_with(&obb));
    }

    #[test]
    fn obb_face_edge_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                45.0_f64.to_radians(),
            )),
        );
        let other_obb = OrientedBoxCollider::new(
            Vector3::new(0.7, 0.7, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );

        assert!(obb.collides_with(&other_obb));
        assert!(other_obb.collides_with(&obb));
    }

    #[test]
    fn obb_dont_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                45.0_f64.to_radians(),
            )),
        );
        let other_obb = OrientedBoxCollider::new(
            Vector3::new(1.71, 1.71, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );

        assert!(!obb.collides_with(&other_obb));
        assert!(!other_obb.collides_with(&obb));
    }

    #[test]
    fn point_corner_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let point = PointCollider::new(Vector3::new(1.0, 1.0, 1.0));

        assert!(obb.collides_with(&point));
        assert!(point.collides_with(&obb));
    }

    #[test]
    fn point_edge_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let point = PointCollider::new(Vector3::new(1.0, 0.0, 1.0));

        assert!(obb.collides_with(&point));
        assert!(point.collides_with(&obb));
    }

    #[test]
    fn point_face_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let point = PointCollider::new(Vector3::new(1.0, 0.0, 0.0));

        assert!(obb.collides_with(&point));
        assert!(point.collides_with(&obb));
    }

    #[test]
    fn point_dont_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                45.0_f64.to_radians(),
            )),
        );
        let point = PointCollider::new(Vector3::new(0.71, 0.71, 0.0));

        assert!(!obb.collides_with(&point));
        assert!(!point.collides_with(&obb));
    }

    #[test]
    fn sphere_corner_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let sphere = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 1.733);

        assert!(obb.collides_with(&sphere));
        assert!(sphere.collides_with(&obb));
    }

    #[test]
    fn sphere_corner_dont_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let sphere = SphereCollider::new(Vector3::new(2.0, 2.0, 2.0), 1.73);

        assert!(!obb.collides_with(&sphere));
        assert!(!sphere.collides_with(&obb));
    }

    #[test]
    fn sphere_edge_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let sphere = SphereCollider::new(Vector3::new(1.70, 1.70, 0.0), 1.0);

        assert!(obb.collides_with(&sphere));
        assert!(sphere.collides_with(&obb));
    }

    #[test]
    fn sphere_edge_dont_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let sphere = SphereCollider::new(Vector3::new(1.71, 1.71, 0.0), 1.0);

        assert!(!obb.collides_with(&sphere));
        assert!(!sphere.collides_with(&obb));
    }

    #[test]
    fn sphere_face_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let sphere = SphereCollider::new(Vector3::new(1.0, 0.0, 0.0), 1.0);

        assert!(obb.collides_with(&sphere));
        assert!(sphere.collides_with(&obb));
    }

    #[test]
    fn sphere_inside_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
                90.0_f64.to_radians(),
            )),
        );
        let sphere = SphereCollider::new(Vector3::new(0.0, 0.0, 0.0), 1.0);

        assert!(obb.collides_with(&sphere));
        assert!(sphere.collides_with(&obb));
    }

    #[test]
    fn aabb_corner_corner_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(obb.collides_with(&aabb));
        assert!(aabb.collides_with(&obb));
    }

    #[test]
    fn aabb_edge_edge_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(obb.collides_with(&aabb));
        assert!(aabb.collides_with(&obb));
    }

    #[test]
    fn aabb_face_face_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(f64::EPSILON, f64::EPSILON, f64::EPSILON),
            Vector3::new(1.0, 1.0, 1.0),
            Quaternion::normalized(0.0, 0.0, 0.0, 1.0),
        );
        let aabb =
            AlignedBoxCollider::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(obb.collides_with(&aabb));
        assert!(aabb.collides_with(&obb));
    }

    #[test]
    fn aabb_dont_collide() {
        let obb = OrientedBoxCollider::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Quaternion::from_euler(Vector3::new(
                0.0_f64.to_radians(),
                0.0_f64.to_radians(),
                45.0_f64.to_radians(),
            )),
        );
        let other_obb =
            AlignedBoxCollider::new(Vector3::new(1.21, 1.21, 0.0), Vector3::new(1.0, 1.0, 1.0));

        assert!(!obb.collides_with(&other_obb));
        assert!(!other_obb.collides_with(&obb));
    }
}
