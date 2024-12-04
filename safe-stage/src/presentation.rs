use crate::types::CVector3;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

#[cfg(feature = "ffi")]
#[repr(C)]
pub struct TriangleBuffer {
    data: *const CVector3,
    len: usize,
}

#[cfg(feature = "ffi")]
impl TriangleBuffer {
    pub fn from_vec(data: Vec<CVector3>) -> Self {
        let boxed = data.into_boxed_slice();
        let len = boxed.len();
        let ptr = Box::into_raw(boxed) as *const CVector3;
        TriangleBuffer { data: ptr, len }
    }

    pub fn data(&self) -> &[CVector3] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }

    #[no_mangle]
    pub extern "C" fn trianglebuffer_drop(self) {
        // Dropped after going out of scope
    }
}

#[cfg(feature = "ffi")]
impl Drop for TriangleBuffer {
    fn drop(&mut self) {
        unsafe {
            let _slice = Box::from_raw(std::slice::from_raw_parts_mut(
                self.data as *mut CVector3,
                self.len,
            ));
        }
    }
}

#[cfg(not(feature = "ffi"))]
pub struct TriangleBuffer {
    data: Vec<CVector3>,
}

#[cfg(not(feature = "ffi"))]
impl TriangleBuffer {
    pub fn from_vec(data: Vec<CVector3>) -> Self {
        TriangleBuffer { data }
    }

    pub fn data(&self) -> &[CVector3] {
        &self.data
    }
}

pub fn collider_to_triangle_buffer(collider: ColliderGroup<PrimaryCollider>) -> TriangleBuffer {
    let triangles = collider.triangle_buffer(|v| CVector3::from(&v));
    TriangleBuffer::from_vec(triangles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_buffer_without_leaking() {
        let triangles = vec![
            CVector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            CVector3 {
                x: 3.0,
                y: 2.0,
                z: 1.0,
            },
            CVector3 {
                x: 1.0,
                y: 3.0,
                z: 2.0,
            },
        ];

        let expected = triangles.as_slice();
        let actual = TriangleBuffer::from_vec(triangles.clone());
        assert_eq!(expected, actual.data());
    }
}
