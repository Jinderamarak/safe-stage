use crate::resolver::PathResolver;
use models::position::sixaxis::SixAxis;

pub mod down_rotate_find;
pub mod linear;

pub trait StagePathResolver: PathResolver<SixAxis> {}
