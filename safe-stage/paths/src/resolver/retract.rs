use crate::resolver::{DynamicImmovable, DynamicMovable, PathResolver};
use models::position::linear::LinearState;

pub mod linear;

pub trait RetractPathResolver:
    PathResolver<LinearState, DynamicMovable<LinearState>, DynamicImmovable>
{
}
