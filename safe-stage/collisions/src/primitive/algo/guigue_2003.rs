//! Rust implementation of the algorithm described in
//! "Fast and Robust Triangle-Triangle Overlap Test using Orientation Predicates"
//! by Philippe Guigue, Olivier Devillers in 2003
//!
//! Based on the C implementation available here:
//! https://github.com/erich666/jgt-code/blob/1c80455c8aafe61955f61372380d983ce7453e6d/Volume_08/Number_1/Guigue2003/tri_tri_intersect.c

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::too_many_arguments)]

use maths::{Vector2, Vector3};

const COPLANAR_EPSILON: f64 = 1e-16;

pub fn tri_tri_overlap_test_3d(
    p1: Vector3,
    q1: Vector3,
    r1: Vector3,
    p2: Vector3,
    q2: Vector3,
    r2: Vector3,
) -> bool {
    let v1 = p2 - r2;
    let v2 = q2 - r2;
    let n2 = v1.cross(&v2);

    let v1 = p1 - r2;
    let dp1 = v1.dot(&n2);
    let v1 = q1 - r2;
    let dq1 = v1.dot(&n2);
    let v1 = r1 - r2;
    let dr1 = v1.dot(&n2);

    let dp1 = if dp1.abs() < COPLANAR_EPSILON {
        0.0
    } else {
        dp1
    };
    let dq1 = if dq1.abs() < COPLANAR_EPSILON {
        0.0
    } else {
        dq1
    };
    let dr1 = if dr1.abs() < COPLANAR_EPSILON {
        0.0
    } else {
        dr1
    };

    if (dp1 * dq1) > 0.0 && (dp1 * dr1) > 0.0 {
        return false;
    }

    let v1 = q1 - p1;
    let v2 = r1 - p1;
    let n1 = v1.cross(&v2);

    let v1 = p2 - r1;
    let dp2 = v1.dot(&n1);
    let v1 = q2 - r1;
    let dq2 = v1.dot(&n1);
    let v1 = r2 - r1;
    let dr2 = v1.dot(&n1);

    let dp2 = if dp2.abs() < COPLANAR_EPSILON {
        0.0
    } else {
        dp2
    };
    let dq2 = if dq2.abs() < COPLANAR_EPSILON {
        0.0
    } else {
        dq2
    };
    let dr2 = if dr2.abs() < COPLANAR_EPSILON {
        0.0
    } else {
        dr2
    };

    if (dp2 * dq2) > 0.0 && (dp2 * dr2) > 0.0 {
        return false;
    }

    if dp1 > 0.0 {
        if dq1 > 0.0 {
            tri_tri_3d(r1, p1, q1, p2, r2, q2, dp2, dr2, dq2, n1)
        } else if dr1 > 0.0 {
            tri_tri_3d(q1, r1, p1, p2, r2, q2, dp2, dr2, dq2, n1)
        } else {
            tri_tri_3d(p1, q1, r1, p2, q2, r2, dp2, dq2, dr2, n1)
        }
    } else if dp1 < 0.0 {
        if dq1 < 0.0 {
            tri_tri_3d(r1, p1, q1, p2, q2, r2, dp2, dq2, dr2, n1)
        } else if dr1 < 0.0 {
            tri_tri_3d(q1, r1, p1, p2, q2, r2, dp2, dq2, dr2, n1)
        } else {
            tri_tri_3d(p1, q1, r1, p2, r2, q2, dp2, dr2, dq2, n1)
        }
    } else {
        if dq1 < 0.0 {
            if dr1 >= 0.0 {
                tri_tri_3d(q1, r1, p1, p2, r2, q2, dp2, dr2, dq2, n1)
            } else {
                tri_tri_3d(p1, q1, r1, p2, q2, r2, dp2, dq2, dr2, n1)
            }
        } else if dq1 > 0.0 {
            if dr1 > 0.0 {
                tri_tri_3d(p1, q1, r1, p2, r2, q2, dp2, dr2, dq2, n1)
            } else {
                tri_tri_3d(q1, r1, p1, p2, q2, r2, dp2, dq2, dr2, n1)
            }
        } else {
            if dr1 > 0.0 {
                tri_tri_3d(r1, p1, q1, p2, q2, r2, dp2, dq2, dr2, n1)
            } else if dr1 < 0.0 {
                tri_tri_3d(r1, p1, q1, p2, r2, q2, dp2, dr2, dq2, n1)
            } else {
                coplanar_tri_tri_3d(p1, q1, r1, p2, q2, r2, n1)
            }
        }
    }
}

fn coplanar_tri_tri_3d(
    p1: Vector3,
    q1: Vector3,
    r1: Vector3,
    p2: Vector3,
    q2: Vector3,
    r2: Vector3,
    normal_1: Vector3,
) -> bool {
    let n_x = if normal_1.x() < 0.0 {
        -normal_1.x()
    } else {
        normal_1.x()
    };
    let n_y = if normal_1.y() < 0.0 {
        -normal_1.y()
    } else {
        normal_1.y()
    };
    let n_z = if normal_1.z() < 0.0 {
        -normal_1.z()
    } else {
        normal_1.z()
    };

    let (p1, q1, r1, p2, q2, r2) = if (n_x > n_z) && (n_x >= n_y) {
        (
            Vector2::new(q1.z(), q1.y()),
            Vector2::new(p1.z(), p1.y()),
            Vector2::new(r1.z(), r1.y()),
            Vector2::new(q2.z(), q2.y()),
            Vector2::new(p2.z(), p2.y()),
            Vector2::new(r2.z(), r2.y()),
        )
    } else if (n_y > n_z) && (n_y >= n_x) {
        (
            Vector2::new(q1.x(), q1.z()),
            Vector2::new(p1.x(), p1.z()),
            Vector2::new(r1.x(), r1.z()),
            Vector2::new(q2.x(), q2.z()),
            Vector2::new(p2.x(), p2.z()),
            Vector2::new(r2.x(), r2.z()),
        )
    } else {
        (
            Vector2::new(p1.x(), p1.y()),
            Vector2::new(q1.x(), q1.y()),
            Vector2::new(r1.x(), r1.y()),
            Vector2::new(p2.x(), p2.y()),
            Vector2::new(q2.x(), q2.y()),
            Vector2::new(r2.x(), r2.y()),
        )
    };

    tri_tri_overlap_test_2d(p1, q1, r1, p2, q2, r2)
}

fn tri_tri_overlap_test_2d(
    p1: Vector2,
    q1: Vector2,
    r1: Vector2,
    p2: Vector2,
    q2: Vector2,
    r2: Vector2,
) -> bool {
    if orient_2d(p1, q1, r1) < 0.0 {
        if orient_2d(p2, q2, r2) < 0.0 {
            ccw_tri_tri_intersection_2d(p1, r1, q1, p2, r2, q2)
        } else {
            ccw_tri_tri_intersection_2d(p1, r1, q1, p2, q2, r2)
        }
    } else {
        if orient_2d(p2, q2, r2) < 0.0 {
            ccw_tri_tri_intersection_2d(p1, q1, r1, p2, r2, q2)
        } else {
            ccw_tri_tri_intersection_2d(p1, q1, r1, p2, q2, r2)
        }
    }
}

#[inline(always)]
fn check_min_max(
    p1: Vector3,
    q1: Vector3,
    r1: Vector3,
    p2: Vector3,
    q2: Vector3,
    r2: Vector3,
) -> bool {
    let v1 = p2 - q1;
    let v2 = p1 - q1;
    let n1 = v1.cross(&v2);
    let v1 = q2 - q1;
    if v1.dot(&n1) > 0.0 {
        return false;
    }

    let v1 = p2 - p1;
    let v2 = r1 - p1;
    let n1 = v1.cross(&v2);
    let v1 = r2 - p1;

    v1.dot(&n1) <= 0.0
}

#[inline(always)]
fn tri_tri_3d(
    p1: Vector3,
    q1: Vector3,
    r1: Vector3,
    p2: Vector3,
    q2: Vector3,
    r2: Vector3,
    dp2: f64,
    dq2: f64,
    dr2: f64,
    n1: Vector3,
) -> bool {
    if dp2 > 0.0 {
        if dq2 > 0.0 {
            check_min_max(p1, r1, q1, r2, p2, q2)
        } else if dr2 > 0.0 {
            check_min_max(p1, r1, q1, q2, r2, p2)
        } else {
            check_min_max(p1, q1, r1, p2, q2, r2)
        }
    } else if dp2 < 0.0 {
        if dq2 < 0.0 {
            check_min_max(p1, q1, r1, r2, p2, q2)
        } else if dr2 < 0.0 {
            check_min_max(p1, q1, r1, q2, r2, p2)
        } else {
            check_min_max(p1, r1, q1, p2, q2, r2)
        }
    } else {
        if dq2 < 0.0 {
            if dr2 >= 0.0 {
                check_min_max(p1, r1, q1, q2, r2, p2)
            } else {
                check_min_max(p1, q1, r1, p2, q2, r2)
            }
        } else if dq2 > 0.0 {
            if dr2 > 0.0 {
                check_min_max(p1, r1, q1, p2, q2, r2)
            } else {
                check_min_max(p1, q1, r1, q2, r2, p2)
            }
        } else {
            if dr2 > 0.0 {
                check_min_max(p1, q1, r1, r2, p2, q2)
            } else if dr2 < 0.0 {
                check_min_max(p1, r1, q1, r2, p2, q2)
            } else {
                coplanar_tri_tri_3d(p1, q1, r1, p2, q2, r2, n1)
            }
        }
    }
}

#[inline(always)]
fn orient_2d(a: Vector2, b: Vector2, c: Vector2) -> f64 {
    (a.x() - c.x()) * (b.y() - c.y()) - (a.y() - c.y()) * (b.x() - c.x())
}

#[inline(always)]
fn intersection_test_vertex(
    p1: Vector2,
    q1: Vector2,
    r1: Vector2,
    p2: Vector2,
    q2: Vector2,
    r2: Vector2,
) -> bool {
    if orient_2d(r2, p2, q1) >= 0.0 {
        if orient_2d(r2, q2, q1) <= 0.0 {
            if orient_2d(p1, p2, q1) > 0.0 {
                orient_2d(p1, q2, q1) <= 0.0
            } else {
                if orient_2d(p1, p2, r1) >= 0.0 {
                    orient_2d(q1, r1, p2) >= 0.0
                } else {
                    false
                }
            }
        } else {
            if orient_2d(p1, q2, q1) <= 0.0 {
                if orient_2d(r2, q2, r1) <= 0.0 {
                    orient_2d(q1, r1, q2) >= 0.0
                } else {
                    false
                }
            } else {
                false
            }
        }
    } else {
        if orient_2d(r2, p2, r1) >= 0.0 {
            if orient_2d(q1, r1, r2) >= 0.0 {
                orient_2d(p1, p2, r1) >= 0.0
            } else {
                if orient_2d(q1, r1, q2) >= 0.0 {
                    orient_2d(r2, r1, q2) >= 0.0
                } else {
                    false
                }
            }
        } else {
            false
        }
    }
}

#[inline(always)]
fn intersection_test_edge(
    p1: Vector2,
    q1: Vector2,
    r1: Vector2,
    p2: Vector2,
    _: Vector2,
    r2: Vector2,
) -> bool {
    if orient_2d(r2, p2, q1) >= 0.0 {
        if orient_2d(p1, p2, q1) >= 0.0 {
            orient_2d(p1, q1, r2) >= 0.0
        } else {
            if orient_2d(q1, r1, p2) >= 0.0 {
                orient_2d(r1, p1, p2) >= 0.0
            } else {
                false
            }
        }
    } else {
        if orient_2d(r2, p2, r1) >= 0.0 {
            if orient_2d(p1, p2, r1) >= 0.0 {
                if orient_2d(p1, r1, r2) >= 0.0 {
                    true
                } else {
                    orient_2d(q1, r1, r2) >= 0.0
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn ccw_tri_tri_intersection_2d(
    p1: Vector2,
    q1: Vector2,
    r1: Vector2,
    p2: Vector2,
    q2: Vector2,
    r2: Vector2,
) -> bool {
    if orient_2d(p2, q2, p1) >= 0.0 {
        if orient_2d(q2, r2, p1) >= 0.0 {
            if orient_2d(r2, p2, p1) >= 0.0 {
                true
            } else {
                intersection_test_edge(p1, q1, r1, p2, q2, r2)
            }
        } else {
            if orient_2d(r2, p2, p1) >= 0.0 {
                intersection_test_edge(p1, q1, r1, r2, p2, q2)
            } else {
                intersection_test_vertex(p1, q1, r1, p2, q2, r2)
            }
        }
    } else {
        if orient_2d(q2, r2, p1) >= 0.0 {
            if orient_2d(r2, p2, p1) >= 0.0 {
                intersection_test_edge(p1, q1, r1, q2, r2, p2)
            } else {
                intersection_test_vertex(p1, q1, r1, q2, r2, p2)
            }
        } else {
            intersection_test_vertex(p1, q1, r1, r2, p2, q2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tri_dataset;
    use super::*;

    #[test]
    fn compare_reference() {
        let triangles = tri_dataset::triangles();

        let mut yes = 0;
        let mut no = 0;

        for i in 0..(triangles.len() / 2) {
            let a = &triangles[i * 2];
            let b = &triangles[i * 2 + 1];

            let (p1, q1, r1) = a.points();
            let (p2, q2, r2) = b.points();

            if tri_tri_overlap_test_3d(*p1, *q1, *r1, *p2, *q2, *r2) {
                yes += 1;
            } else {
                no += 1;
            }
        }

        //  Values from the reference implementation in C
        assert_eq!(294, yes);
        assert_eq!(170, no);
    }
}
