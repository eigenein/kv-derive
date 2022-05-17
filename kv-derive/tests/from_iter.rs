use kv_derive::FromIter;

#[test]
fn empty_ok() -> Result<(), kv_derive_impl::error::Error> {
    #[derive(Debug, Default, PartialEq, FromIter)]
    struct Empty;

    assert_eq!(Empty::from_iter(vec![])?, Empty);
    Ok(())
}
