#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearState {
    /// Equivalent to `Partial(0.0)`.
    None,
    /// Equivalent to `Partial(1.0)`.
    Full,
    /// A value between `0.0` (`Empty`) and `1.0` (`Full`).
    Partial(f64),
}

impl LinearState {
    pub fn relative(position: f64) -> LinearState {
        assert!((0.0..=1.0).contains(&position));
        match position {
            0.0 => LinearState::None,
            1.0 => LinearState::Full,
            r => LinearState::Partial(r),
        }
    }

    pub fn as_relative(&self) -> f64 {
        match self {
            Self::None => 0.0,
            Self::Full => 1.0,
            Self::Partial(p) => *p,
        }
    }

    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        if t == 0.0 {
            return *self;
        }
        if t == 1.0 {
            return *other;
        }

        let r = self.as_relative() + (other.as_relative() - self.as_relative()) * t;
        LinearState::relative(r)
    }
}
