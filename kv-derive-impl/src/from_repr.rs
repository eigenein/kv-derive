use std::str::FromStr;

use anyhow::{Context, Result};

/// Converts a string representation back to the typed value.
/// The opposite of [`crate::to_repr::ToRepr`].
pub trait FromRepr: Sized {
    fn from_repr(string: &str) -> Result<Self>;
}

impl<T: FromStr> FromRepr for T
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    fn from_repr(string: &str) -> Result<T> {
        T::from_str(string).with_context(|| format!("failed to convert: `{}`", string))
    }
}
