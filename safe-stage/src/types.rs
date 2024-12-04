use maths::Vector3;
use models::position::linear::LinearState;
use models::position::sixaxis::SixAxis;
use paths::path::PathResult;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct CLinearState {
    pub t: f64,
}

impl From<&CLinearState> for LinearState {
    fn from(c: &CLinearState) -> Self {
        LinearState::relative(c.t)
    }
}

impl From<&LinearState> for CLinearState {
    fn from(s: &LinearState) -> Self {
        CLinearState { t: s.as_relative() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct CVector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<&CVector3> for Vector3 {
    fn from(c: &CVector3) -> Self {
        Vector3::new(c.x, c.y, c.z)
    }
}

impl From<&Vector3> for CVector3 {
    fn from(v: &Vector3) -> Self {
        CVector3 {
            x: v.x(),
            y: v.y(),
            z: v.z(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct CSixAxis {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}

impl From<&CSixAxis> for SixAxis {
    fn from(c: &CSixAxis) -> Self {
        SixAxis {
            pos: Vector3::new(c.x, c.y, c.z),
            rot: Vector3::new(c.rx, c.ry, c.rz),
        }
    }
}

impl From<&SixAxis> for CSixAxis {
    fn from(s: &SixAxis) -> Self {
        CSixAxis {
            x: s.pos.x(),
            y: s.pos.y(),
            z: s.pos.z(),
            rx: s.rot.x(),
            ry: s.rot.y(),
            rz: s.rot.z(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "ffi", repr(u8))]
pub enum PathResultState {
    InvalidStart,
    Path,
    UnreachableEnd,
}

macro_rules! path_result_impl {
    ($name:ident, $dropper:ident, $node:ident, $base:ty) => {
        #[cfg(feature = "ffi")]
        #[derive(Debug, Clone, PartialEq)]
        #[repr(C)]
        pub struct $name {
            state: PathResultState,
            nodes: *const $node,
            len: usize,
        }

        #[cfg(feature = "ffi")]
        impl $name {
            pub fn from_vec(state: PathResultState, nodes: Vec<$node>) -> Self {
                let boxed = nodes.into_boxed_slice();
                let len = boxed.len();
                let ptr = Box::into_raw(boxed) as *const $node;
                $name {
                    state,
                    nodes: ptr,
                    len,
                }
            }

            pub fn nodes(&self) -> &[$node] {
                unsafe { std::slice::from_raw_parts(self.nodes, self.len) }
            }

            #[no_mangle]
            pub extern "C" fn $dropper(self) {
                // Dropped after going out of scope
            }
        }

        #[cfg(feature = "ffi")]
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    let _slice = Box::from_raw(std::slice::from_raw_parts_mut(
                        self.nodes as *mut $node,
                        self.len,
                    ));
                }
            }
        }

        #[cfg(not(feature = "ffi"))]
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            state: PathResultState,
            nodes: Vec<$node>,
        }

        #[cfg(not(feature = "ffi"))]
        impl $name {
            pub fn from_vec(state: PathResultState, nodes: Vec<$node>) -> Self {
                $name { state, nodes }
            }

            pub fn nodes(&self) -> &[$node] {
                &self.nodes
            }
        }

        impl From<PathResult<$base>> for $name {
            fn from(value: PathResult<$base>) -> Self {
                use paths::path::PathResult;
                use $crate::types::PathResultState;

                let tag = match &value {
                    PathResult::InvalidStart(_) => PathResultState::InvalidStart,
                    PathResult::Path(_) => PathResultState::Path,
                    PathResult::UnreachableEnd(_) => PathResultState::UnreachableEnd,
                };

                let nodes = match value {
                    PathResult::InvalidStart(_) | PathResult::UnreachableEnd(None) => vec![],
                    PathResult::Path(path) => path.iter().map($node::from).collect(),
                    PathResult::UnreachableEnd(Some(path)) => {
                        path.iter().map($node::from).collect()
                    }
                };

                $name::from_vec(tag, nodes)
            }
        }
    };
}

path_result_impl!(
    CPathResultSixAxis,
    cpathresultsixaxis_drop,
    CSixAxis,
    SixAxis
);
path_result_impl!(
    CPathResultLinearState,
    cpathresultlinearstate_drop,
    CLinearState,
    LinearState
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn populated_path_without_leak() {
        let nodes = vec![
            CLinearState { t: 0.0 },
            CLinearState { t: 1.0 },
            CLinearState { t: 2.0 },
            CLinearState { t: 3.0 },
        ];
        let path = CPathResultLinearState::from_vec(PathResultState::Path, nodes);

        let expected = [
            CLinearState { t: 0.0 },
            CLinearState { t: 1.0 },
            CLinearState { t: 2.0 },
            CLinearState { t: 3.0 },
        ];
        let actual = path.nodes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_path_without_leak() {
        let nodes = vec![];
        let path = CPathResultLinearState::from_vec(PathResultState::UnreachableEnd, nodes);

        let expected: [CLinearState; 0] = [];
        let actual = path.nodes();
        assert_eq!(expected, actual);
    }
}
