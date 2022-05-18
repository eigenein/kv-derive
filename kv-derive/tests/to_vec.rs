//! Cases which aren't covered in the README.
use kv_derive::prelude::*;

#[test]
fn empty_ok() {
    #[derive(IntoVec)]
    struct Empty;

    assert_eq!(Empty.into_vec(), Vec::new());
}
