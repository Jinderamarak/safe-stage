use models::position::sixaxis::SixAxis;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum PathResult<P> {
    /// The start point is invalid.
    InvalidStart(P),
    /// Entire path from the start to the end.
    Path(Vec<P>),
    /// Partial path from the start, end cannot be reached.
    UnreachableEnd(Option<Vec<P>>),
}

impl<P> PathResult<P> {
    pub fn nodes(&self) -> usize {
        match self {
            PathResult::InvalidStart(_) | PathResult::UnreachableEnd(None) => 0,
            PathResult::Path(path) | PathResult::UnreachableEnd(Some(path)) => path.len(),
        }
    }

    pub fn map<F>(&self, func: F) -> PathResult<P>
    where
        P: Clone,
        F: Fn(&[P]) -> Vec<P>,
    {
        match self {
            PathResult::InvalidStart(p) => PathResult::InvalidStart(p.clone()),
            PathResult::UnreachableEnd(None) => PathResult::UnreachableEnd(None),
            PathResult::UnreachableEnd(Some(path)) => {
                PathResult::UnreachableEnd(Some(func(path.as_slice())))
            }
            PathResult::Path(path) => PathResult::Path(func(path.as_slice())),
        }
    }
}

impl PathResult<SixAxis> {
    pub fn time_length(&self, speed: &SixAxis) -> f64 {
        match self {
            PathResult::InvalidStart(_) | PathResult::UnreachableEnd(None) => 0.0,
            PathResult::Path(path) | PathResult::UnreachableEnd(Some(path)) => {
                path.windows(2).fold(0.0, |acc, pair| match pair {
                    [from, to] => {
                        let time = from.time_to(to, speed);
                        acc + time
                    }
                    _ => acc,
                })
            }
        }
    }
}
