use paths::resolver::retract::RetractPathResolver;
use paths::resolver::stage::StagePathResolver;

macro_rules! concrete_resolver_impl {
    ($name:ident, $t:ident) => {
        #[cfg(feature = "ffi")]
        #[repr(C)]
        pub struct $name(*mut std::ffi::c_void);

        #[cfg(feature = "ffi")]
        impl $name {
            pub fn new(resolver: impl $t + 'static) -> Self {
                let boxed: Box<Box<dyn $t>> = Box::new(Box::new(resolver));
                let raw: *mut Box<dyn $t> = Box::into_raw(boxed);
                Self(raw as *mut std::ffi::c_void)
            }

            #[allow(dead_code)]
            pub fn get_ref(&self) -> &dyn $t {
                let ptr = self.0 as *mut Box<dyn $t>;
                unsafe { &**ptr }
            }

            #[allow(dead_code)]
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
                    let _r = Box::from_raw(ptr);
                };
            }
        }

        #[cfg(not(feature = "ffi"))]
        pub struct $name(Box<dyn $t>);

        #[cfg(not(feature = "ffi"))]
        impl $name {
            pub fn new(resolver: impl $t + 'static) -> Self {
                Self(Box::new(resolver))
            }

            #[allow(dead_code)]
            pub fn get_ref(&self) -> &dyn $t {
                &*self.0
            }

            #[allow(dead_code)]
            pub fn get_mut(&mut self) -> &mut dyn $t {
                &mut *self.0
            }
        }
    };
}

concrete_resolver_impl!(ConcreteRetractResolver, RetractPathResolver);
concrete_resolver_impl!(ConcreteStageResolver, StagePathResolver);

#[cfg(test)]
mod tests {
    use super::*;
    
    
    
    use models::collider::ModelCollider;
    use models::movable::Movable;
    use models::position::linear::LinearState;
    use models::position::sixaxis::SixAxis;
    use paths::path::PathResult;
    use paths::resolver::{PathResolver, StateUpdateError};

    struct TestRetractResolver;
    impl RetractPathResolver for TestRetractResolver {}
    impl PathResolver<LinearState> for TestRetractResolver {
        fn update_state(
            &mut self,
            _new: &LinearState,
            _movable: &dyn Movable<LinearState>,
            _immovable: &dyn ModelCollider,
        ) -> Result<(), StateUpdateError> {
            unreachable!()
        }

        fn resolve_path(
            &self,
            _from: &LinearState,
            _to: &LinearState,
            _movable: &dyn Movable<LinearState>,
            _immovable: &dyn ModelCollider,
        ) -> PathResult<LinearState> {
            unreachable!()
        }
    }

    #[test]
    fn concrete_retract_resolver() {
        let _concrete = ConcreteRetractResolver::new(TestRetractResolver);
    }

    struct TestStageResolver;
    impl StagePathResolver for TestStageResolver {}
    impl PathResolver<SixAxis> for TestStageResolver {
        fn update_state(
            &mut self,
            _new: &SixAxis,
            _movable: &dyn Movable<SixAxis>,
            _immovable: &dyn ModelCollider,
        ) -> Result<(), StateUpdateError> {
            unreachable!()
        }

        fn resolve_path(
            &self,
            _from: &SixAxis,
            _to: &SixAxis,
            _movable: &dyn Movable<SixAxis>,
            _immovable: &dyn ModelCollider,
        ) -> PathResult<SixAxis> {
            unreachable!()
        }
    }

    #[test]
    fn concrete_stage_resolver() {
        let _concrete = ConcreteStageResolver::new(TestStageResolver);
    }
}
