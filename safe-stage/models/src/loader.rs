use collisions::primitive::TriangleCollider;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tinystl::StlData;

use maths::Vector3;
pub use tinystl::Error as StlError;

pub fn load_stl_from_file(path: impl AsRef<Path>) -> Result<Vec<TriangleCollider>, StlError> {
    let file = std::fs::File::open(path).map_err(StlError::Io)?;
    let reader = BufReader::new(file);
    load_stl(reader)
}

pub fn load_stl_from_bytes(data: &[u8]) -> Result<Vec<TriangleCollider>, StlError> {
    let reader = BufReader::new(data);
    load_stl(reader)
}

pub fn load_stl(reader: impl BufRead) -> Result<Vec<TriangleCollider>, StlError> {
    let data = StlData::read_buffer(reader)?;
    let mut triangles = Vec::new();
    for triangle in &data.triangles {
        //  TODO: Handle degenerate triangles
        triangles.push(TriangleCollider::new(
            array_to_vector(&triangle.v1),
            array_to_vector(&triangle.v2),
            array_to_vector(&triangle.v3),
        ));
    }

    triangles.shrink_to_fit();
    Ok(triangles)
}

fn array_to_vector(v: &[f32; 3]) -> Vector3 {
    Vector3::new(v[0] as f64, v[1] as f64, v[2] as f64)
}
