use crate::collides_group_impl;
use crate::common::{Collides, Rotation, Transformation, Translation};
use crate::primitive::algo::guigue_2003;
use maths::{Quaternion, Vector3};

//  Common vertex buffer adds too much complexity for not a lot of gain,
//  and it introduces indirect memory access

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct TriangleCollider {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl PartialEq for TriangleCollider {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

impl TriangleCollider {
    pub fn new(a: Vector3, b: Vector3, c: Vector3) -> Self {
        debug_assert!(
            a != b && b != c && c != a,
            "All vertices of the triangle must be different"
        );

        Self { a, b, c }
    }

    pub fn points(&self) -> (&Vector3, &Vector3, &Vector3) {
        (&self.a, &self.b, &self.c)
    }
}

impl Collides<Self> for TriangleCollider {
    fn collides_with(&self, other: &Self) -> bool {
        let (p1, q1, r1) = self.points();
        let (p2, q2, r2) = other.points();

        guigue_2003::tri_tri_overlap_test_3d(*p1, *q1, *r1, *p2, *q2, *r2)
    }
}

collides_group_impl!(TriangleCollider, TriangleCollider);

impl Rotation for TriangleCollider {
    fn rotate(&self, rotation: &Quaternion) -> Self {
        let center = self.a + self.b + self.c / 3.0;
        Self::new(
            self.a.rotate_around(rotation, &center),
            self.b.rotate_around(rotation, &center),
            self.c.rotate_around(rotation, &center),
        )
    }

    fn rotate_around(&self, rotation: &Quaternion, pivot: &Vector3) -> Self {
        Self::new(
            self.a.rotate_around(rotation, pivot),
            self.b.rotate_around(rotation, pivot),
            self.c.rotate_around(rotation, pivot),
        )
    }
}

impl Translation for TriangleCollider {
    fn translate(&self, translation: &Vector3) -> Self {
        Self::new(
            self.a + translation,
            self.b + translation,
            self.c + translation,
        )
    }
}

impl Transformation for TriangleCollider {
    fn transform(&self, rotation: &Quaternion, pivot: &Vector3, translation: &Vector3) -> Self {
        Self::new(
            self.a.rotate_around(rotation, pivot) + translation,
            self.b.rotate_around(rotation, pivot) + translation,
            self.c.rotate_around(rotation, pivot) + translation,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotate_vec(v: &Vector3) -> Vector3 {
        Vector3::new(v.y(), v.z(), v.x())
    }

    fn rotate_tri(t: TriangleCollider) -> TriangleCollider {
        let (a, b, c) = t.points();
        TriangleCollider::new(rotate_vec(a), rotate_vec(b), rotate_vec(c))
    }

    #[test]
    fn triangles_dont_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(-1.0, 2.0, 1.0),
            Vector3::new(-1.0, 1.0, -1.0),
        );

        let collide = t1.collides_with(&t2);
        assert!(!collide);
    }

    #[test]
    fn triangles_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(1.0, -1.0, 1.0),
            Vector3::new(-1.0, 2.0, 1.0),
            Vector3::new(-1.0, 1.0, -1.0),
        );

        let collides = t1.collides_with(&t2);
        assert!(collides);
    }

    #[test]
    fn triangles_corner_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(0.0, 1.0, 1.0),
            Vector3::new(-1.0, 2.0, 1.0),
            Vector3::new(-1.0, 1.0, -1.0),
        );

        let collides = t1.collides_with(&t2);
        assert!(collides);
    }

    #[test]
    fn triangles_parallel_dont_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(2.0, -1.0, 1.0),
            Vector3::new(0.0, 2.0, 1.0),
            Vector3::new(0.0, 1.0, -1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(1.0, -1.0, 1.0),
            Vector3::new(-1.0, 2.0, 1.0),
            Vector3::new(-1.0, 1.0, -1.0),
        );

        let collides = t1.collides_with(&t2);
        assert!(!collides);
    }

    #[test]
    fn triangles_coplanar_dont_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-2.0, 0.0, -1.0),
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(0.0, 1.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(2.0, 0.0, 3.0),
            Vector3::new(1.0, 1.0, 2.0),
            Vector3::new(0.0, -1.0, 1.0),
        );

        let collides = t1.collides_with(&t2);
        assert!(!collides);
    }

    #[test]
    fn triangles_coplanar_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-2.0, 0.0, -1.0),
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(0.0, 1.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(1.0, 0.0, 2.0),
            Vector3::new(-1.0, 1.0, 0.0),
            Vector3::new(0.0, -1.0, 1.0),
        );

        let collides = t1.collides_with(&t2);
        assert!(collides);
    }

    #[test]
    fn triangles_coplanar_xz_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-2.0, 3.0, -1.0),
            Vector3::new(-1.0, 3.0, 1.0),
            Vector3::new(1.0, 3.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(1.0, 3.0, 0.0),
            Vector3::new(-1.0, 3.0, -1.0),
            Vector3::new(0.0, 3.0, 2.0),
        );

        let collides = t1.collides_with(&t2);
        assert!(collides);
    }

    #[test]
    fn triangles_coplanar_yz_dont_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(-1.0, 2.0, 0.0),
            Vector3::new(-1.0, 1.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(-1.0, 1.0, 2.0),
            Vector3::new(-1.0, -2.0, -1.0),
            Vector3::new(-1.0, -1.0, 2.0),
        );

        let collides = t1.collides_with(&t2);
        assert!(!collides);
    }

    #[test]
    fn triangles_coplanar_xy_contained_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-2.0, 1.0, 1.0),
            Vector3::new(3.0, 3.0, 1.0),
            Vector3::new(2.0, -2.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(-1.0, 1.0, 1.0),
            Vector3::new(2.0, 2.0, 1.0),
            Vector3::new(2.0, -1.0, 1.0),
        );

        let collides = t1.collides_with(&t2);
        assert!(collides);
    }

    #[test]
    fn triangles_coplanar_axis_aligned_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-2.0, 3.0, -1.0),
            Vector3::new(-1.0, 3.0, 1.0),
            Vector3::new(1.0, 3.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(1.0, 3.0, 0.0),
            Vector3::new(-1.0, 3.0, -1.0),
            Vector3::new(0.0, 3.0, 2.0),
        );

        let t1 = rotate_tri(t1);
        let t2 = rotate_tri(t2);
        let collides = t1.collides_with(&t2);
        assert!(collides, "yz plane");

        let t1 = rotate_tri(t1);
        let t2 = rotate_tri(t2);
        let collides = t1.collides_with(&t2);
        assert!(collides, "xy plane");

        let t1 = rotate_tri(t1);
        let t2 = rotate_tri(t2);
        let collides = t1.collides_with(&t2);
        assert!(collides, "xz plane");
    }

    #[test]
    fn triangles_coplanar_axis_aligned_dont_collide() {
        let t1 = TriangleCollider::new(
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(-1.0, 2.0, 0.0),
            Vector3::new(-1.0, 1.0, 1.0),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(-1.0, 1.0, 2.0),
            Vector3::new(-1.0, -2.0, -1.0),
            Vector3::new(-1.0, -1.0, 2.0),
        );

        let t1 = rotate_tri(t1);
        let t2 = rotate_tri(t2);
        let collides = t1.collides_with(&t2);
        assert!(!collides, "xy plane");

        let t1 = rotate_tri(t1);
        let t2 = rotate_tri(t2);
        let collides = t1.collides_with(&t2);
        assert!(!collides, "xz plane");

        let t1 = rotate_tri(t1);
        let t2 = rotate_tri(t2);
        let collides = t1.collides_with(&t2);
        assert!(!collides, "yz plane");
    }

    #[test]
    fn real_case_1() {
        let t1 = TriangleCollider::new(
            Vector3::new(
                -0.2009113106545759,
                -0.41227065485460146,
                -0.028230926021933556,
            ),
            Vector3::new(
                -0.194_150_630_305_032_2,
                -0.41656731155258997,
                -0.022_949_626_669_287_68,
            ),
            Vector3::new(
                -0.2009113106545759,
                -0.415_742_260_334_318_7,
                -0.02252211794257164,
            ),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(-0.20000000298023224, -0.5, 0.30000001192092896),
            Vector3::new(-0.20000000298023224, -0.5, -0.30000001192092896),
            Vector3::new(
                -0.20000000298023224,
                -1.100_000_023_841_858,
                0.30000001192092896,
            ),
        );

        let collides = t1.collides_with(&t2);
        assert!(!collides);
    }
    #[test]
    fn real_case_2() {
        let t1 = TriangleCollider::new(
            Vector3::new(
                -0.2402728080418659,
                -0.376_573_202_130_621_5,
                -0.005_281_300_283_968_449,
            ),
            Vector3::new(
                -0.2402728080418659,
                -0.377_252_319_758_063_4,
                -0.004_647_048_655_897_379,
            ),
            Vector3::new(
                -0.24073851030736693,
                -0.372276545432633,
                0.00000000000000022592187177130827,
            ),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(
                -1.100_000_023_841_858,
                0.15000000596046448,
                -0.400_000_005_960_464_5,
            ),
            Vector3::new(
                -1.100_000_023_841_858,
                -0.800_000_011_920_929,
                -0.400_000_005_960_464_5,
            ),
            Vector3::new(-1.149_999_976_158_142, -0.75, -0.400_000_005_960_464_5),
        );

        let collides = t1.collides_with(&t2);
        assert!(!collides);
    }

    #[test]
    fn real_case_3() {
        let t1 = TriangleCollider::new(
            Vector3::new(
                -0.2009113106545759,
                -0.41227065485460146,
                -0.028230926021933556,
            ),
            Vector3::new(
                -0.194_150_630_305_032_2,
                -0.41656731155258997,
                -0.022_949_626_669_287_68,
            ),
            Vector3::new(
                -0.2009113106545759,
                -0.415_742_260_334_318_7,
                -0.02252211794257164,
            ),
        );
        let t2 = TriangleCollider::new(
            Vector3::new(-0.20000000298023224, -0.5, 0.30000001192092896),
            Vector3::new(-0.20000000298023224, -0.5, -0.30000001192092896),
            Vector3::new(
                -0.20000000298023224,
                -1.100_000_023_841_858,
                0.30000001192092896,
            ),
        );

        let collides = t1.collides_with(&t2);
        assert!(!collides);
    }
}
