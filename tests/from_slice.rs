use anyhow::Result;

use kv_derive::FromSlice;

#[test]
fn empty_ok() -> Result<()> {
    #[derive(Debug, Default, PartialEq, FromSlice)]
    struct Empty;

    assert_eq!(Empty::from_slice(&vec![])?, Empty);
    Ok(())
}
