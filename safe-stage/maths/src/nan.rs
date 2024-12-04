pub trait NaNExtension {
    fn map_nan(self, default: Self) -> Self;
}

impl NaNExtension for f64 {
    #[inline]
    fn map_nan(self, default: Self) -> Self {
        if self.is_nan() {
            default
        } else {
            self
        }
    }
}
