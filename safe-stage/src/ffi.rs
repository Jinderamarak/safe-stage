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

macro_rules! ffi_vec_for_type {
    ($name:ident, $drop:ident, $t:ty) => {
        ffi_vec_for_type!(, $name, $drop, $t);
    };
    ($v:vis, $name:ident, $drop:ident, $t:ty) => {
        #[cfg(feature = "ffi")]
        #[repr(C)]
        $v struct $name {
            data: *const $t,
            len: usize,
        }

        #[cfg(feature = "ffi")]
        impl $name {
            pub fn from_vec(data: Vec<$t>) -> Self {
                let boxed = data.into_boxed_slice();
                let len = boxed.len();
                let ptr = Box::into_raw(boxed) as *const $t;
                Self { data: ptr, len }
            }

            pub fn data(&self) -> &[$t] {
                unsafe { std::slice::from_raw_parts(self.data, self.len) }
            }

            #[no_mangle]
            pub extern "C" fn $drop(self) {
                // Dropped after going out of scope
            }
        }

        #[cfg(feature = "ffi")]
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    let _slice = Box::from_raw(std::slice::from_raw_parts_mut(
                        self.data as *mut $t,
                        self.len,
                    ));
                }
            }
        }

        #[cfg(not(feature = "ffi"))]
        $v struct $name {
            data: Vec<$t>,
        }

        #[cfg(not(feature = "ffi"))]
        impl $name {
            pub fn from_vec(data: Vec<$t>) -> Self {
                Self { data }
            }

            pub fn data(&self) -> &[$t] {
                &self.data
            }
        }
    };
}

pub(crate) use ffi_vec_for_type;

#[cfg(test)]
mod tests {
    use super::*;

    opaque_ffi_for_type!(TestVecU8, Vec<u8>);

    #[test]
    fn correct_opaque_mutabilty() {
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

    ffi_vec_for_type!(TestVecU16, test_vec_u16_drop, u16);

    #[test]
    fn correct_ffi_vec() {
        let values = TestVecU16::from_vec(vec![1, 2, 3]);
        let expected = [1, 2, 3];
        let actual = values.data();
        assert_eq!(&expected, actual);
    }
}
