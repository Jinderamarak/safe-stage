/// Collision between two objects
pub trait Collides<T> {
    fn collides_with(&self, other: &T) -> bool;
}
