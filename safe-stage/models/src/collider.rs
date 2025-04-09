use collisions::common::Collides;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

pub trait ModelCollider: Collides<ColliderGroup<PrimaryCollider>> + Send + Sync {}

impl ModelCollider for PrimaryCollider {}
impl ModelCollider for ColliderGroup<PrimaryCollider> {}
