use kv_derive::ToVec;

#[test]
fn empty_ok() {
    #[derive(ToVec)]
    struct Empty;

    assert_eq!(Empty.to_vec(), Vec::new());
}
