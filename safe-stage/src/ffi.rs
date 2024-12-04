macro_rules! opaque_ffi_for_type {
    ($name:ident, $t:ty) => {
        opaque_ffi_for_type!(, $name, $t);
    };
    ($v:vis, $name:ident, $t:ty) => {
        #[cfg(feature = "ffi")]
        #[repr(C)]
        $v struct $name(*mut std::ffi::c_void);

        #[cfg(feature = "ffi")]
        impl $name {
            fn from_inner(inner: $t) -> Self {
                let inner = Box::into_raw(Box::new(inner)) as *mut std::ffi::c_void;
                Self(inner)
            }

            #[allow(dead_code)]
            pub fn inner(&self) -> &$t {
                unsafe { &*(self.0 as *const $t) }
            }

            #[allow(dead_code)]
            pub fn inner_mut(&mut self) -> &mut $t {
                unsafe { &mut *(self.0 as *mut $t) }
            }
        }

        #[cfg(feature = "ffi")]
        impl Drop for $name {
            fn drop(&mut self) {
                let _inner = unsafe { Box::from_raw(self.0 as *mut $t) };
            }
        }

        #[cfg(not(feature = "ffi"))]
        $v struct $name($t);

        #[cfg(not(feature = "ffi"))]
        impl $name {
            pub fn from_inner(inner: $t) -> Self {
                Self(inner)
            }

            #[allow(dead_code)]
            pub fn inner(&self) -> &$t {
                &self.0
            }

            #[allow(dead_code)]
            pub fn inner_mut(&mut self) -> &mut $t {
                &mut self.0
            }
        }
    };
}

pub(crate) use opaque_ffi_for_type;

#[cfg(test)]
mod tests {
    use super::*;

    opaque_ffi_for_type!(TestVecU8, Vec<u8>);

    #[test]
    fn correct_mutabilty() {
        let mut values = TestVecU8::from_inner(vec![1, 2, 3]);

        values.inner_mut().push(4);
        let expected = [1, 2, 3, 4];
        let actual = values.inner();
        assert_eq!(&expected, actual.as_slice());

        values.inner_mut().pop();
        let expected = [1, 2, 3];
        let actual = values.inner();
        assert_eq!(&expected, actual.as_slice());
    }
}
