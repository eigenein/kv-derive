/// Converts a value to its string representation.
/// The opposite of [`crate::from_repr::FromRepr`].
pub trait ToRepr {
    fn to_repr(&self) -> String;
}

impl<T: ToString> ToRepr for T {
    fn to_repr(&self) -> String {
        self.to_string()
    }
}
