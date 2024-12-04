use crate::movable::Movable;
use crate::parts::holder::Holder;
use crate::position::sixaxis::SixAxis;
use std::sync::Arc;

pub trait Stage: Movable<SixAxis> + Send + Sync {
    fn as_arc(&self) -> Arc<dyn Movable<SixAxis> + Send + Sync>;
    fn swap_holder(&mut self, holder: Option<Box<dyn Holder>>);
    fn active_holder(&self) -> Option<&dyn Holder>;
    fn active_holder_mut(&mut self) -> Option<&mut (dyn Holder + 'static)>;
}
