use crate::from_repr::FromRepr;
use crate::result::Result;

/// Responsible for consuming the string value and modifying itself accordingly.
pub trait Consumer<T>: Sized {
    /// Initialize a new instance of `T` from its string representation.
    fn init(value: &str) -> Result<Self>;

    /// Consume or accumulate the new value into itself.
    ///
    /// May consume one or more entries.
    fn consume(&mut self, value: &str) -> Result<()>;
}

impl<T: FromRepr> Consumer<T> for T {
    fn init(value: &str) -> Result<Self> {
        T::from_repr(value)
    }

    fn consume(&mut self, value: &str) -> Result<()> {
        *self = Self::init(value)?;
        Ok(())
    }
}

impl<T: FromRepr> Consumer<T> for Option<T> {
    fn init(value: &str) -> Result<Self> {
        Ok(Some(T::from_repr(value)?))
    }

    fn consume(&mut self, value: &str) -> Result<()> {
        *self = Some(T::from_repr(value)?);
        Ok(())
    }
}

impl<T: FromRepr> Consumer<T> for Vec<T> {
    fn init(value: &str) -> Result<Self> {
        Ok(vec![T::from_repr(value)?])
    }

    fn consume(&mut self, value: &str) -> Result<()> {
        self.push(T::from_repr(value)?);
        Ok(())
    }
}
