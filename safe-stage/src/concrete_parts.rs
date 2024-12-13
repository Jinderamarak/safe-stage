use models::parts::chamber::Chamber;
use models::parts::equipment::Equipment;
use models::parts::holder::Holder;
use models::parts::retract::Retract;
use models::parts::stage::Stage;

macro_rules! concrete_part_impl {
    ($name:ident, $t:ident) => {
        #[cfg(feature = "ffi")]
        #[repr(C)]
        pub struct $name(*mut std::ffi::c_void);

        #[cfg(feature = "ffi")]
        impl $name {
            pub fn new(part: impl $t + 'static) -> Self {
                let boxed: Box<Box<dyn $t>> = Box::new(Box::new(part));
                let raw: *mut Box<dyn $t> = Box::into_raw(boxed);
                Self(raw as *mut std::ffi::c_void)
            }

            pub fn get_ref(&self) -> &dyn $t {
                let ptr = self.0 as *mut Box<dyn $t>;
                unsafe { &**ptr }
            }

            pub fn get_mut(&mut self) -> &mut dyn $t {
                let ptr = self.0 as *mut Box<dyn $t>;
                unsafe { &mut **ptr }
            }
        }

        #[cfg(feature = "ffi")]
        impl Drop for $name {
            fn drop(&mut self) {
                let ptr = self.0 as *mut Box<dyn $t>;
                unsafe {
                    let _p = Box::from_raw(ptr);
                };
            }
        }

        #[cfg(not(feature = "ffi"))]
        pub struct $name(Box<dyn $t>);

        #[cfg(not(feature = "ffi"))]
        impl $name {
            pub fn new(part: impl $t + 'static) -> Self {
                Self(Box::new(part))
            }

            pub fn get_ref(&self) -> &dyn $t {
                &*self.0
            }

            pub fn get_mut(&mut self) -> &mut dyn $t {
                &mut *self.0
            }
        }
    };
}

concrete_part_impl!(ConcreteChamber, Chamber);
concrete_part_impl!(ConcreteEquipment, Equipment);
concrete_part_impl!(ConcreteHolder, Holder);
concrete_part_impl!(ConcreteRetract, Retract);
concrete_part_impl!(ConcreteStage, Stage);

#[cfg(test)]
mod tests {
    use super::*;
    use collisions::complex::bvh_sphere_recursive::BvhSphereRecursive;
    use collisions::complex::group::ColliderGroup;
    use collisions::PrimaryCollider;
    use models::movable::Movable;
    use models::position::linear::LinearState;
    use models::position::sixaxis::SixAxis;
    use std::sync::Arc;

    struct TestChamber;
    impl Chamber for TestChamber {
        fn full(&self) -> ColliderGroup<PrimaryCollider> {
            unreachable!()
        }
        fn less_obstructive(&self) -> ColliderGroup<PrimaryCollider> {
            unreachable!()
        }
        fn non_obstructive(&self) -> ColliderGroup<PrimaryCollider> {
            unreachable!()
        }
    }

    #[test]
    fn concrete_chamber() {
        let _concrete = ConcreteChamber::new(TestChamber);
    }

    struct TestEquipment;
    impl Equipment for TestEquipment {
        fn collider(&self) -> ColliderGroup<BvhSphereRecursive> {
            unreachable!()
        }
    }

    #[test]
    fn concrete_equipment() {
        let _concrete = ConcreteEquipment::new(TestEquipment);
    }

    struct TestHolder;
    impl Holder for TestHolder {
        fn cloned(&self) -> Box<dyn Holder> {
            unreachable!()
        }
        fn collider(&self) -> ColliderGroup<BvhSphereRecursive> {
            unreachable!()
        }
        fn swap_sample(&mut self, _sample: Option<PrimaryCollider>) {
            unreachable!()
        }
    }

    #[test]
    fn concrete_holder() {
        let _concrete = ConcreteHolder::new(TestHolder);
    }

    struct TestRetract;
    impl Movable<LinearState> for TestRetract {
        fn move_to(&self, _position: &LinearState) -> ColliderGroup<PrimaryCollider> {
            unreachable!()
        }
    }
    impl Retract for TestRetract {
        fn as_arc(&self) -> Arc<dyn Movable<LinearState> + Send + Sync> {
            unreachable!()
        }
    }

    #[test]
    fn concrete_retract() {
        let _concrete = ConcreteRetract::new(TestRetract);
    }

    struct TestStage;
    impl Movable<SixAxis> for TestStage {
        fn move_to(&self, _position: &SixAxis) -> ColliderGroup<PrimaryCollider> {
            unreachable!()
        }
    }
    impl Stage for TestStage {
        fn as_arc(&self) -> Arc<dyn Movable<SixAxis> + Send + Sync> {
            unreachable!()
        }
        fn swap_holder(&mut self, _holder: Option<Box<dyn Holder>>) {
            unreachable!()
        }
        fn active_holder(&self) -> Option<&dyn Holder> {
            unreachable!()
        }
        fn active_holder_mut(&mut self) -> Option<&mut (dyn Holder + 'static)> {
            unreachable!()
        }
    }

    #[test]
    fn concrete_stage() {
        let _concrete = ConcreteStage::new(TestStage);
    }
}
