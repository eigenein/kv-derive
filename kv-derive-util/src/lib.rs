pub mod chrono;

use kv_derive::{FromRepr, IntoRepr};

/// Allows to represent a boolean as an integer.
#[derive(FromRepr, IntoRepr, Debug, Default)]
pub struct BooleanAsU8(pub u8);

impl From<bool> for BooleanAsU8 {
    fn from(value: bool) -> Self {
        Self(value as u8)
    }
}
impl From<BooleanAsU8> for bool {
    fn from(value: BooleanAsU8) -> Self {
        value.0 != 0
    }
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
