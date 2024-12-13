use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

pub trait Chamber: Send + Sync {
    fn full(&self) -> ColliderGroup<PrimaryCollider>;
    fn less_obstructive(&self) -> ColliderGroup<PrimaryCollider>;
    fn non_obstructive(&self) -> ColliderGroup<PrimaryCollider>;
}
