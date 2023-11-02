use crate::common::Vector3;

pub trait Projectable {
    fn project(&self, axis: Vector3) -> (f64, f64);
    fn intersects(&self, other: &impl Projectable, axis: Vector3) -> bool {
        let (self_min, self_max) = self.project(axis);
        let (other_min, other_max) = other.project(axis);

        self_max >= other_min && self_min <= other_max
    }
}
