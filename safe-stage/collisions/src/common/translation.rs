use maths::Vector3;

/// # Object translation
/// A translation of an object.
pub trait Translation {
    /// Returns object moved by given translation.
    fn translate(&self, translation: &Vector3) -> Self;
}
