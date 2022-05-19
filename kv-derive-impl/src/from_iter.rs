use crate::result::Result;

/// Constructs the structure from the iterator of key-value pairs.
pub trait FromIter: Sized {
    fn from_iter<'a>(iter: impl IntoIterator<Item = (&'a str, &'a str)>) -> Result<Self>;
}
