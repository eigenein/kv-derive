use kv_derive::IntoVec;
use kv_derive_impl::IntoVec;

#[test]
fn empty_ok() {
    #[derive(IntoVec)]
    struct Empty;

    assert_eq!(Empty.into_vec(), Vec::new());
}
