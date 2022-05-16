# `kv-derive`

[![Last commit](https://img.shields.io/github/last-commit/eigenein/kv-derive?logo=github)](https://github.com/eigenein/kv-derive/commits/master)
[![Build status](https://github.com/eigenein/kv-derive/actions/workflows/check.yaml/badge.svg)](https://github.com/eigenein/kv-derive/actions)

## Example

```rust
use kv_derive::ToVec;

#[derive(ToVec)]
struct Foo {
    bar: i32,
    qux: String,
}

let foo = Foo { bar: 42, qux: "qux".into() };
assert_eq!(foo.to_vec(), vec![
    ("bar", "42".into()),
    ("qux", "qux".into()),
]);
```
