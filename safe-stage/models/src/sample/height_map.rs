use collisions::primitive::TriangleCollider;
use maths::{Vector2, Vector3};

//  marching squares algorithm

pub fn height_map_to_sample_model(
    height_map: &[f64],
    size_x: usize,
    size_y: usize,
    real_size: &Vector2,
    base_height: f64,
) -> Vec<TriangleCollider> {
    assert!(size_x > 0 && size_y > 0);
    assert_eq!(height_map.len(), size_x * size_y);
    assert!(base_height.is_finite());

    let offset = Vector2::new(-real_size.x() / 2.0, -real_size.y() / 2.0);
    let scale = Vector2::new(real_size.x() / size_x as f64, real_size.y() / size_y as f64);

    let mut triangles = Vec::new();
    for x in 0..size_x - 1 {
        for y in 0..size_y - 1 {
            let tl_h = height_map[xy_to_i(size_x, x, y)];
            let tr_h = height_map[xy_to_i(size_x, x + 1, y)];
            let bl_h = height_map[xy_to_i(size_x, x, y + 1)];
            let br_h = height_map[xy_to_i(size_x, x + 1, y + 1)];

            let flats = square_to_triangle(
                tl_h > base_height,
                tr_h > base_height,
                bl_h > base_height,
                br_h > base_height,
            );
            for (a, b, c) in flats.iter() {
                let a = flat_to_actual_point(
                    a,
                    point_height(tl_h, tr_h, bl_h, br_h, base_height, a),
                    x,
                    y,
                    &offset,
                    &scale,
                );
                let b = flat_to_actual_point(
                    b,
                    point_height(tl_h, tr_h, bl_h, br_h, base_height, b),
                    x,
                    y,
                    &offset,
                    &scale,
                );
                let c = flat_to_actual_point(
                    c,
                    point_height(tl_h, tr_h, bl_h, br_h, base_height, c),
                    x,
                    y,
                    &offset,
                    &scale,
                );

                triangles.push(TriangleCollider::new(a, b, c));
            }
        }
    }

    triangles.shrink_to_fit();
    triangles
}

#[inline]
const fn xy_to_i(size_x: usize, x: usize, y: usize) -> usize {
    y * size_x + x
}

#[inline]
fn flat_to_actual_point(
    (flat, _): &(Vector2, bool),
    height: f64,
    x: usize,
    y: usize,
    offset: &Vector2,
    scale: &Vector2,
) -> Vector3 {
    let x = x as f64 * scale.x() + flat.x() * scale.x() + offset.x();
    let y = y as f64 * scale.y() + flat.y() * scale.y() + offset.y();
    let z = height;
    Vector3::new(x, y, z)
}

const TL: Vector2 = Vector2::new(0.0, 0.0);
const TR: Vector2 = Vector2::new(1.0, 0.0);
const BL: Vector2 = Vector2::new(0.0, 1.0);
const BR: Vector2 = Vector2::new(1.0, 1.0);
const TOP: Vector2 = Vector2::new(0.5, 0.0);
const BOTTOM: Vector2 = Vector2::new(0.5, 1.0);
const LEFT: Vector2 = Vector2::new(0.0, 0.5);
const RIGHT: Vector2 = Vector2::new(1.0, 0.5);

/// ```markdown
/// (0, 0)  (1, 0)
///   tl      tr
///     +---+
///     |   |
///     +---+
///   bl      br
/// (0, 1)  (1, 1)
/// ```
#[inline]
#[allow(clippy::type_complexity)]
fn square_to_triangle(
    tl: bool,
    tr: bool,
    bl: bool,
    br: bool,
) -> Box<[((Vector2, bool), (Vector2, bool), (Vector2, bool))]> {
    match (tl, tr, bl, br) {
        (false, false, false, false) => Box::new([]),
        (false, false, true, false) => Box::new([((LEFT, false), (BL, true), (BOTTOM, false))]),
        (false, false, false, true) => Box::new([((BR, true), (RIGHT, false), (BOTTOM, false))]),
        (false, false, true, true) => Box::new([
            ((BL, true), (RIGHT, false), (LEFT, false)),
            ((BL, true), (BR, true), (RIGHT, false)),
        ]),
        (false, true, false, false) => Box::new([((TR, true), (TOP, false), (RIGHT, false))]),
        (false, true, true, false) => Box::new([
            ((TR, true), (TOP, false), (RIGHT, false)),
            ((BL, true), (BOTTOM, false), (LEFT, false)),
            ((TOP, false), (LEFT, false), (BOTTOM, false)),
            ((TOP, false), (BOTTOM, false), (RIGHT, false)),
        ]),
        (false, true, false, true) => Box::new([
            ((TR, true), (TOP, false), (BOTTOM, false)),
            ((TR, true), (BOTTOM, false), (BR, true)),
        ]),
        (false, true, true, true) => Box::new([
            ((BR, true), (TR, true), (TOP, false)),
            ((BR, true), (TOP, false), (LEFT, false)),
            ((BR, true), (LEFT, false), (BL, true)),
        ]),
        (true, false, false, false) => Box::new([((TL, true), (LEFT, false), (TOP, false))]),
        (true, false, true, false) => Box::new([
            ((TL, true), (BOTTOM, false), (TOP, false)),
            ((TL, true), (BL, true), (BOTTOM, false)),
        ]),
        (true, false, false, true) => Box::new([
            ((TL, true), (LEFT, false), (TOP, false)),
            ((BR, true), (RIGHT, false), (BOTTOM, false)),
            ((TOP, false), (LEFT, false), (BOTTOM, false)),
            ((TOP, false), (BOTTOM, false), (RIGHT, false)),
        ]),
        (true, false, true, true) => Box::new([
            ((BL, true), (TOP, false), (TL, true)),
            ((BL, true), (RIGHT, false), (TOP, false)),
            ((BL, true), (BR, true), (RIGHT, false)),
        ]),
        (true, true, false, false) => Box::new([
            ((TL, true), (LEFT, false), (RIGHT, false)),
            ((TL, true), (RIGHT, false), (TR, true)),
        ]),
        (true, true, true, false) => Box::new([
            ((TL, true), (RIGHT, false), (TR, true)),
            ((TL, true), (BOTTOM, false), (RIGHT, false)),
            ((TL, true), (BL, true), (BOTTOM, false)),
        ]),
        (true, true, false, true) => Box::new([
            ((TR, true), (TL, true), (LEFT, false)),
            ((TR, true), (LEFT, false), (BOTTOM, false)),
            ((TR, true), (BOTTOM, false), (BR, true)),
        ]),
        (true, true, true, true) => Box::new([
            ((TL, true), (BL, true), (BR, true)),
            ((TL, true), (BR, true), (TR, true)),
        ]),
    }
}

fn point_height(
    tl: f64,
    tr: f64,
    bl: f64,
    br: f64,
    base: f64,
    (p, inner): &(Vector2, bool),
) -> f64 {
    if !*inner {
        return base;
    }

    let x = p.x();
    let y = p.y();
    let h1 = tl * (1.0 - x) + tr * x;
    let h2 = bl * (1.0 - x) + br * x;
    h1 * (1.0 - y) + h2 * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_point_height() {
        let (tl, tr, bl, br) = (1.0, 2.0, 3.0, 4.0);
        let cases = [
            (Vector2::new(0.0, 0.0), tl),
            (Vector2::new(1.0, 0.0), tr),
            (Vector2::new(0.0, 1.0), bl),
            (Vector2::new(1.0, 1.0), br),
            (Vector2::new(0.5, 0.0), (tl + tr) / 2.0),
            (Vector2::new(0.0, 0.5), (tl + bl) / 2.0),
            (Vector2::new(1.0, 0.5), (tr + br) / 2.0),
            (Vector2::new(0.5, 1.0), (bl + br) / 2.0),
        ];

        for (point, expected) in cases {
            let actual = point_height(tl, tr, bl, br, 0.0, &(point, true));
            assert_eq!(expected, actual);
        }
    }
}
