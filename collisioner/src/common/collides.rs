/// # Object collision
/// Collision detection implementations for different collider combinations.
pub trait Collides<T> {
    /// Returns true if the collider collides with the other collider.
    fn collides_with(&self, other: &T) -> bool;
}
