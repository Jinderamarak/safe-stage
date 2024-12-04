use crate::movable::Movable;
use crate::position::linear::LinearState;
use std::sync::Arc;

pub trait Retract: Movable<LinearState> + Send + Sync {
    fn as_arc(&self) -> Arc<dyn Movable<LinearState> + Send + Sync>;
}
