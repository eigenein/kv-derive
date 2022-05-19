use std::str::FromStr;

use crate::error::Error;
use crate::result::Result;

/// Converts a string representation back to the typed value.
/// The opposite of [`crate::into_repr::IntoRepr`].
pub trait FromRepr: Sized {
    /// Converts a scalar value from its textual representation.
    fn from_repr(string: &str) -> Result<Self>;
}

macro_rules! impl_repr {
    ($type:ty, $error:expr) => {
        impl FromRepr for $type {
            fn from_repr(string: &str) -> Result<$type> {
                <$type>::from_str(string).map_err($error)
            }
        }
    };
}

impl_repr!(bool, Error::ParseBoolError);

impl_repr!(i8, Error::ParseIntError);
impl_repr!(u8, Error::ParseIntError);
impl_repr!(i16, Error::ParseIntError);
impl_repr!(u16, Error::ParseIntError);
impl_repr!(i32, Error::ParseIntError);
impl_repr!(u32, Error::ParseIntError);
impl_repr!(i64, Error::ParseIntError);
impl_repr!(u64, Error::ParseIntError);
impl_repr!(i128, Error::ParseIntError);
impl_repr!(u128, Error::ParseIntError);
impl_repr!(isize, Error::ParseIntError);
impl_repr!(usize, Error::ParseIntError);

impl_repr!(f32, Error::ParseFloatError);
impl_repr!(f64, Error::ParseFloatError);

impl_repr!(String, Error::Infallible);

impl_repr!(std::net::IpAddr, Error::AddrParseError);
impl_repr!(std::net::SocketAddr, Error::AddrParseError);
