use std::intrinsics::unlikely;

pub trait NaNExtension {
    /// Maps `NaN` value to the given default value.
    fn map_nan(self, default: Self) -> Self;
}

impl NaNExtension for f64 {
    #[inline]
    fn map_nan(self, default: Self) -> Self {
        if unlikely(self.is_nan()) {
            default
        } else {
            self
        }
    }
}
