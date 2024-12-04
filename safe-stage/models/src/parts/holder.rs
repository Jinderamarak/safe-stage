use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

pub trait Holder: Send + Sync {
    fn cloned(&self) -> Box<dyn Holder>;
    fn collider(&self) -> ColliderGroup<PrimaryCollider>;
    fn swap_sample(&mut self, sample: Option<PrimaryCollider>);
}
