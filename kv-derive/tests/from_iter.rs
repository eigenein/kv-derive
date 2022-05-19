//! Cases which aren't covered in the README.

use kv_derive::error::Error;
use kv_derive::prelude::*;
use kv_derive::FromIter;

#[test]
fn empty_ok() -> Result<(), Error> {
    #[derive(Debug, Default, PartialEq, FromIter)]
    struct Empty;

    assert_eq!(Empty::from_iter(vec![])?, Empty);
    Ok(())
}
