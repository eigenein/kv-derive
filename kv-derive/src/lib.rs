#![doc = include_str!("../../README.md")]

pub mod prelude;

pub use kv_derive_impl::error::Error;
pub use kv_derive_impl::result::Result;

pub use crate::prelude::*;

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../../README.md"));
}
