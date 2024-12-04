use crate::common::Collides;
use crate::complex::BvhSphere;

#[cfg(feature = "rayon-group")]
use rayon::prelude::*;

#[macro_export]
macro_rules! collider_group {
    ($($i:expr),*) => {
        $crate::complex::group::ColliderGroup(vec![ $($i),* ])
    }
}

pub use collider_group;
use maths::Vector3;

pub struct ColliderGroup<T>(pub Vec<T>);

impl<T> ColliderGroup<T> {
    pub fn extended(mut self, other: ColliderGroup<T>) -> Self {
        self.0.extend(other.0);
        self
    }

    pub fn extend(&mut self, other: ColliderGroup<T>) {
        self.0.extend(other.0);
    }
}

impl ColliderGroup<BvhSphere> {
    #[cfg(feature = "rayon-group")]
    pub fn into_bvh(self) -> BvhSphere {
        self.0
            .into_par_iter()
            .map(Some)
            .reduce(
                || None,
                |a, b| match (a, b) {
                    (Some(a), Some(b)) => Some(a.concat(b)),
                    (Some(x), None) | (None, Some(x)) => Some(x),
                    (None, None) => None,
                },
            )
            .unwrap()
    }

    #[cfg(not(feature = "rayon-group"))]
    pub fn into_bvh(self) -> BvhSphere {
        self.0
            .into_iter()
            .reduce(|a, b| a.concat(b))
            .expect("ColliderGroup cannot be empty")
    }

    pub fn triangle_buffer<T, M>(&self, mapper: M) -> Vec<T>
    where
        T: Send,
        M: (Fn(Vector3) -> T) + Send + Sync,
    {
        #[cfg(feature = "rayon-group")]
        let data_iter = self.0.par_iter();

        #[cfg(not(feature = "rayon-group"))]
        let data_iter = self.0.iter();

        data_iter
            .flat_map(|t| t.triangle_buffer())
            .map(mapper)
            .collect()
    }
}

#[cfg(feature = "rayon-group")]
impl<A, B> Collides<ColliderGroup<A>> for ColliderGroup<B>
where
    A: Sync,
    B: Collides<A> + Sync,
{
    fn collides_with(&self, a: &ColliderGroup<A>) -> bool {
        self.0
            .par_iter()
            .any(|b| a.0.par_iter().any(|a| b.collides_with(a)))
    }
}

#[cfg(not(feature = "rayon-group"))]
impl<A, B> Collides<ColliderGroup<A>> for ColliderGroup<B>
where
    B: Collides<A>,
{
    fn collides_with(&self, a: &ColliderGroup<A>) -> bool {
        self.0
            .iter()
            .any(|b| a.0.iter().any(|a| b.collides_with(a)))
    }
}

#[cfg(feature = "rayon-group")]
#[macro_export]
macro_rules! collides_group_impl {
    ($($t1:ty, $t2:ty)*) => (
        #[allow(unused_imports)]
        use rayon::prelude::*;

        $(
            impl $crate::common::Collides<$t1> for $crate::complex::group::ColliderGroup<$t2> {
                #[inline]
                fn collides_with(&self, other: &$t1) -> bool {
                    self.0.par_iter().any(|item| item.collides_with(other))
                }
            }
            impl $crate::common::Collides<$crate::complex::group::ColliderGroup<$t2>> for $t1 {
                #[inline]
                fn collides_with(&self, other: &$crate::complex::group::ColliderGroup<$t2>) -> bool {
                    other.collides_with(self)
                }
            }
        )*
    )
}

#[cfg(not(feature = "rayon-group"))]
#[macro_export]
macro_rules! collides_group_impl {
    ($($t1:ty, $t2:ty)*) => ($(
        impl $crate::common::Collides<$t1> for $crate::complex::group::ColliderGroup<$t2> {
            #[inline]
            fn collides_with(&self, other: &$t1) -> bool {
                self.0.iter().any(|item| item.collides_with(other))
            }
        }
        impl $crate::common::Collides<$crate::complex::group::ColliderGroup<$t2>> for $t1 {
            #[inline]
            fn collides_with(&self, other: &$crate::complex::group::ColliderGroup<$t2>) -> bool {
                other.collides_with(self)
            }
        }
    )*)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::TriangleCollider;
    use crate::PrimaryCollider;

    struct Col;
    impl Collides<Col> for Col {
        fn collides_with(&self, _: &Col) -> bool {
            true
        }
    }

    collides_group_impl!(Col, Col);

    #[test]
    fn collider_group() {
        let a = Col;
        let b = Col;
        let c = Col;
        let group = collider_group!(a, b, c);
        assert!(Col.collides_with(&group));
        assert!(group.collides_with(&Col));
    }

    #[test]
    fn vertices_keep_ordering() {
        let collider = collider_group!(
            PrimaryCollider::build(&[
                TriangleCollider::new(
                    Vector3::new(1.0, 2.0, 3.0),
                    Vector3::new(3.0, 2.0, 1.0),
                    Vector3::new(1.0, 3.0, 2.0),
                ),
                TriangleCollider::new(
                    Vector3::new(5.0, 7.0, 6.0),
                    Vector3::new(9.0, 8.0, 7.0),
                    Vector3::new(6.0, 5.0, 8.0),
                )
            ]),
            PrimaryCollider::build(&[TriangleCollider::new(
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(1.0, 2.0, 2.0),
                Vector3::new(1.0, 2.0, 3.0),
            )])
        );

        let expected = [
            [
                Vector3::new(1.0, 2.0, 3.0),
                Vector3::new(3.0, 2.0, 1.0),
                Vector3::new(1.0, 3.0, 2.0),
            ],
            [
                Vector3::new(5.0, 7.0, 6.0),
                Vector3::new(9.0, 8.0, 7.0),
                Vector3::new(6.0, 5.0, 8.0),
            ],
            [
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(1.0, 2.0, 2.0),
                Vector3::new(1.0, 2.0, 3.0),
            ],
        ];
        let actual = collider.triangle_buffer(|x| x);

        for series in expected {
            let [a, b, c] = series;
            let first = actual.iter().position(|&x| x == a).unwrap();

            assert_eq!(a, actual[first]);
            assert_eq!(b, actual[first + 1]);
            assert_eq!(c, actual[first + 2]);
        }
    }
}
