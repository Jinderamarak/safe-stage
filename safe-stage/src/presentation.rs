use crate::ffi::ffi_vec_for_type;
use crate::types::CVector3;
use collisions::complex::group::ColliderGroup;
use collisions::PrimaryCollider;

ffi_vec_for_type!(pub, TriangleBuffer, trianglebuffer_drop, CVector3);
ffi_vec_for_type!(
    pub,
    TriangleBufferVec,
    trianglebuffervec_drop,
    TriangleBuffer
);

pub fn collider_to_triangle_buffer_per_item(
    group: ColliderGroup<PrimaryCollider>,
) -> TriangleBufferVec {
    let vec = group
        .triangle_buffer_per_item(|v| CVector3::from(&v))
        .into_iter()
        .map(TriangleBuffer::from_vec)
        .collect::<Vec<TriangleBuffer>>();

    TriangleBufferVec::from_vec(vec)
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
