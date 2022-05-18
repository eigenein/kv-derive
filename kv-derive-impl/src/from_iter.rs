use crate::result::Result;

pub trait FromIter {
    fn from_iter<'a>(iter: impl IntoIterator<Item = (&'a str, &'a str)>) -> Result<Self>
    where
        Self: Default;
}
