use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

pub trait Chamber: Send + Sync {
    fn full(&self) -> ColliderGroup<PrimaryCollider>;
    fn without_walls(&self) -> ColliderGroup<PrimaryCollider>;
    fn only_walls(&self) -> ColliderGroup<PrimaryCollider>;
}
