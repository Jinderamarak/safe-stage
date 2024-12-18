use crate::primitive::TriangleCollider;

/// # Treeable collider
/// A trait for colliders which are to be used a bounding shape in the [BvhRecursive].
pub trait Treeable {
    /// Returns a collider which bounds both self and other.
    fn bound_children(&self, other: &Self) -> Self;

    /// Returns a collider which bounds a triangle.
    fn bound_triangle(triangle: &TriangleCollider) -> Self;
}
