use kv_derive::ToVec;

#[test]
fn empty_to_key_values_ok() {
    #[derive(ToVec)]
    struct Empty;

    assert_eq!(Empty.to_vec(), Vec::new());
}

#[test]
fn test_scalar() {
    #[derive(ToVec)]
    struct Scalar {
        scalar: i32,
    }

    assert_eq!(Scalar { scalar: 42 }.to_vec(), vec![("scalar", "42".to_string())]);
}
