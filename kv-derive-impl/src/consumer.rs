use crate::result::Result;
use crate::FromRepr;

pub trait Consumer {
    fn consume(&mut self, value: &str) -> Result<()>;
}

impl<T: FromRepr> Consumer for T {
    fn consume(&mut self, value: &str) -> Result<()> {
        *self = T::from_repr(value)?;
        Ok(())
    }
}

impl<T: FromRepr> Consumer for Option<T> {
    fn consume(&mut self, value: &str) -> Result<()> {
        *self = Some(T::from_repr(value)?);
        Ok(())
    }
}
