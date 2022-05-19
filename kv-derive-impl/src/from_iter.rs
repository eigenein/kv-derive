use crate::result::Result;

pub trait FromIter: Sized {
    fn from_iter<'a>(iter: impl IntoIterator<Item = (&'a str, &'a str)>) -> Result<Self>;
}
