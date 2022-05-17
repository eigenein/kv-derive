use crate::result::Result;
use crate::FromRepr;

/// Responsible for consuming the string value and modifying itself accordingly.
///
/// May consume one or more entries.
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

impl<T: FromRepr> Consumer for Vec<T> {
    fn consume(&mut self, value: &str) -> Result<()> {
        self.push(T::from_repr(value)?);
        Ok(())
    }
}
