use crate::primitive::{
    AlignedBoxCollider, Collider, OrientedBoxCollider, PointCollider, SphereCollider,
};
use maths::{Quaternion, Vector3};

/// Starting point for the `Collider` builder.
pub struct WithBuilder;

impl WithBuilder {
    pub(crate) fn start() -> WithBuilder {
        WithBuilder
    }
}

pub trait BuildCollider {
    /// Build a `Collider` from the current state.
    fn build(self) -> Collider;
}

pub struct WithCenter<P> {
    prev: P,
    center: Vector3,
}

pub trait AddCenter
where
    Self: Sized,
{
    /// Set the center of the `Collider`.
    fn center(self, center: Vector3) -> WithCenter<Self> {
        WithCenter { prev: self, center }
    }

    /// Set the center of the `Collider` using the given coordinates.
    fn center_xyz(self, cx: f64, cy: f64, cz: f64) -> WithCenter<Self> {
        WithCenter {
            prev: self,
            center: Vector3::new(cx, cy, cz),
        }
    }
}

pub struct WithRadius<P> {
    prev: P,
    radius: f64,
}

pub trait AddRadius
where
    Self: Sized,
{
    /// Set the radius of the `Collider`.
    fn radius(self, radius: f64) -> WithRadius<Self> {
        WithRadius { prev: self, radius }
    }
}

pub struct WithSize<P> {
    prev: P,
    size: Vector3,
}

pub trait AddSize
where
    Self: Sized,
{
    /// Set the size of the `Collider`.
    fn size(self, size: Vector3) -> WithSize<Self> {
        WithSize { prev: self, size }
    }

    /// Set the size of the `Collider` using the given coordinates.
    fn size_xyz(self, sx: f64, sy: f64, sz: f64) -> WithSize<Self> {
        WithSize {
            prev: self,
            size: Vector3::new(sx, sy, sz),
        }
    }
}

pub struct WithRotation<P> {
    prev: P,
    rotation: Quaternion,
}

pub trait AddRotation
where
    Self: Sized,
{
    /// Set the rotation of the `Collider`.
    fn rotation(self, rotation: Quaternion) -> WithRotation<Self> {
        WithRotation {
            prev: self,
            rotation,
        }
    }

    /// Set the rotation of the `Collider` using the given euler angles.
    fn rotation_euler(self, rx: f64, ry: f64, rz: f64) -> WithRotation<Self> {
        WithRotation {
            prev: self,
            rotation: Quaternion::from_euler(&Vector3::new(rx, ry, rz)),
        }
    }
}

impl AddCenter for WithBuilder {}
impl AddCenter for WithRadius<WithBuilder> {}
impl AddCenter for WithSize<WithBuilder> {}
impl AddCenter for WithRotation<WithBuilder> {}
impl AddCenter for WithSize<WithRotation<WithBuilder>> {}
impl AddCenter for WithRotation<WithSize<WithBuilder>> {}

impl AddRadius for WithBuilder {}
impl AddRadius for WithCenter<WithBuilder> {}

impl AddSize for WithBuilder {}
impl AddSize for WithCenter<WithBuilder> {}
impl AddSize for WithRotation<WithBuilder> {}
impl AddSize for WithCenter<WithRotation<WithBuilder>> {}
impl AddSize for WithRotation<WithCenter<WithBuilder>> {}

impl AddRotation for WithBuilder {}
impl AddRotation for WithCenter<WithBuilder> {}
impl AddRotation for WithSize<WithBuilder> {}
impl AddRotation for WithCenter<WithSize<WithBuilder>> {}
impl AddRotation for WithSize<WithCenter<WithBuilder>> {}

impl BuildCollider for WithCenter<WithBuilder> {
    fn build(self) -> Collider {
        Collider::Point(PointCollider::new(self.center))
    }
}

impl BuildCollider for WithRadius<WithCenter<WithBuilder>> {
    fn build(self) -> Collider {
        Collider::Sphere(SphereCollider::new(self.prev.center, self.radius))
    }
}

impl BuildCollider for WithCenter<WithRadius<WithBuilder>> {
    fn build(self) -> Collider {
        Collider::Sphere(SphereCollider::new(self.center, self.prev.radius))
    }
}

impl BuildCollider for WithSize<WithCenter<WithBuilder>> {
    fn build(self) -> Collider {
        Collider::AlignedBox(AlignedBoxCollider::new(self.prev.center, self.size))
    }
}

impl BuildCollider for WithCenter<WithSize<WithBuilder>> {
    fn build(self) -> Collider {
        Collider::AlignedBox(AlignedBoxCollider::new(self.center, self.prev.size))
    }
}

impl BuildCollider for WithRotation<WithSize<WithCenter<WithBuilder>>> {
    fn build(self) -> Collider {
        Collider::OrientedBox(OrientedBoxCollider::new(
            self.prev.prev.center,
            self.prev.size,
            self.rotation,
        ))
    }
}

impl BuildCollider for WithCenter<WithRotation<WithSize<WithBuilder>>> {
    fn build(self) -> Collider {
        Collider::OrientedBox(OrientedBoxCollider::new(
            self.center,
            self.prev.prev.size,
            self.prev.rotation,
        ))
    }
}

impl BuildCollider for WithSize<WithRotation<WithCenter<WithBuilder>>> {
    fn build(self) -> Collider {
        Collider::OrientedBox(OrientedBoxCollider::new(
            self.prev.prev.center,
            self.size,
            self.prev.rotation,
        ))
    }
}

impl BuildCollider for WithCenter<WithSize<WithRotation<WithBuilder>>> {
    fn build(self) -> Collider {
        Collider::OrientedBox(OrientedBoxCollider::new(
            self.center,
            self.prev.size,
            self.prev.prev.rotation,
        ))
    }
}

impl BuildCollider for WithRotation<WithCenter<WithSize<WithBuilder>>> {
    fn build(self) -> Collider {
        Collider::OrientedBox(OrientedBoxCollider::new(
            self.prev.center,
            self.prev.prev.size,
            self.rotation,
        ))
    }
}

impl BuildCollider for WithSize<WithCenter<WithRotation<WithBuilder>>> {
    fn build(self) -> Collider {
        Collider::OrientedBox(OrientedBoxCollider::new(
            self.prev.center,
            self.size,
            self.prev.prev.rotation,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point() {
        let expected = Collider::Point(PointCollider::new(Vector3::new(1.0, 2.0, 3.0)));
        let actual = WithBuilder::start().center_xyz(1.0, 2.0, 3.0).build();
        assert_eq!(expected, actual);
    }

    #[test]
    fn sphere() {
        let expected = Collider::Sphere(SphereCollider::new(Vector3::new(1.0, 2.0, 3.0), 4.0));
        let sphere1 = WithBuilder::start()
            .center_xyz(1.0, 2.0, 3.0)
            .radius(4.0)
            .build();
        let sphere2 = WithBuilder::start()
            .radius(4.0)
            .center_xyz(1.0, 2.0, 3.0)
            .build();

        assert_eq!(expected, sphere1);
        assert_eq!(expected, sphere2);
    }

    #[test]
    fn aligned_box() {
        let expected = Collider::AlignedBox(AlignedBoxCollider::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
        ));
        let box1 = WithBuilder::start()
            .center_xyz(1.0, 2.0, 3.0)
            .size_xyz(4.0, 5.0, 6.0)
            .build();
        let box2 = WithBuilder::start()
            .size_xyz(4.0, 5.0, 6.0)
            .center_xyz(1.0, 2.0, 3.0)
            .build();

        assert_eq!(expected, box1);
        assert_eq!(expected, box2);
    }

    #[test]
    fn oriented_box() {
        let expected = Collider::OrientedBox(OrientedBoxCollider::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Quaternion::from_euler(&Vector3::new(7.0, 8.0, 9.0)),
        ));
        let box1 = WithBuilder::start()
            .center_xyz(1.0, 2.0, 3.0)
            .size_xyz(4.0, 5.0, 6.0)
            .rotation_euler(7.0, 8.0, 9.0)
            .build();
        let box2 = WithBuilder::start()
            .center_xyz(1.0, 2.0, 3.0)
            .size_xyz(4.0, 5.0, 6.0)
            .rotation_euler(7.0, 8.0, 9.0)
            .build();

        let box3 = WithBuilder::start()
            .size_xyz(4.0, 5.0, 6.0)
            .center_xyz(1.0, 2.0, 3.0)
            .rotation_euler(7.0, 8.0, 9.0)
            .build();
        let box4 = WithBuilder::start()
            .size_xyz(4.0, 5.0, 6.0)
            .rotation_euler(7.0, 8.0, 9.0)
            .center_xyz(1.0, 2.0, 3.0)
            .build();

        let box5 = WithBuilder::start()
            .rotation_euler(7.0, 8.0, 9.0)
            .size_xyz(4.0, 5.0, 6.0)
            .center_xyz(1.0, 2.0, 3.0)
            .build();
        let box6 = WithBuilder::start()
            .rotation_euler(7.0, 8.0, 9.0)
            .center_xyz(1.0, 2.0, 3.0)
            .size_xyz(4.0, 5.0, 6.0)
            .build();

        assert_eq!(expected, box1);
        assert_eq!(expected, box2);
        assert_eq!(expected, box3);
        assert_eq!(expected, box4);
        assert_eq!(expected, box5);
        assert_eq!(expected, box6);
    }
}
