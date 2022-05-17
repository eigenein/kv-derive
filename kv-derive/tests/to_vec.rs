use kv_derive::ToVec;
use kv_derive_impl::ToVec;

#[test]
fn empty_ok() {
    #[derive(ToVec)]
    struct Empty;

    assert_eq!(Empty.to_vec(), Vec::new());
}
