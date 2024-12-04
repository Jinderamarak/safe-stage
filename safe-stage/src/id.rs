#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ffi", repr(C))]
pub struct Id(u64);

impl Id {
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn id_new(id: u64) -> Self {
        Id(id)
    }

    #[cfg(not(feature = "ffi"))]
    pub fn new(id: u64) -> Self {
        Id(id)
    }
}

#[cfg(all(test, feature = "ffi"))]
macro_rules! make_id {
    ($id:expr) => {
        $crate::id::Id::id_new($id)
    };
}

#[cfg(all(test, not(feature = "ffi")))]
macro_rules! make_id {
    ($id:expr) => {
        $crate::id::Id::new($id)
    };
}

#[cfg(test)]
pub(crate) use make_id;
