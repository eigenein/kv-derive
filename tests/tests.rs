use kv_derive::ToKeyValues;

#[test]
fn empty_to_key_values_ok() {
    #[derive(ToKeyValues)]
    struct Empty;

    assert_eq!(Empty.to_key_values(), Vec::new());
}
