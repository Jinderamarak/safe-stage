use crate::resolver::PathResolver;
use models::position::linear::LinearState;

pub mod linear;

pub trait RetractPathResolver: PathResolver<LinearState> {}
