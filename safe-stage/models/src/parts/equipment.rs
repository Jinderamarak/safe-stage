use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

pub trait Equipment: Send + Sync {
    fn collider(&self) -> ColliderGroup<PrimaryCollider>;
}
