//! Cases which aren't covered in the README.
use kv_derive::FromIter;
use kv_derive_impl::FromIter;

#[test]
fn empty_ok() -> Result<(), kv_derive_impl::error::Error> {
    #[derive(Debug, Default, PartialEq, FromIter)]
    struct Empty;

    assert_eq!(Empty::from_iter(vec![])?, Empty);
    Ok(())
}
