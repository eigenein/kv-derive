use anyhow::Result;
use kv_derive::FromIter;

#[test]
fn empty_ok() -> Result<()> {
    #[derive(Debug, Default, PartialEq, FromIter)]
    struct Empty;

    assert_eq!(Empty::from_iter(vec![])?, Empty);
    Ok(())
}
