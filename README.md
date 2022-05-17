# `kv-derive`

Derive `struct` conversions from and to key-value vectors using [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) and [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html).

[![Crates.io](https://img.shields.io/crates/v/kv-derive)](https://crates.io/crates/kv-derive)
[![Last commit](https://img.shields.io/github/last-commit/eigenein/kv-derive?logo=github)](https://github.com/eigenein/kv-derive/commits/master)
[![Build status](https://github.com/eigenein/kv-derive/actions/workflows/check.yaml/badge.svg)](https://github.com/eigenein/kv-derive/actions)
![License: MIT](https://img.shields.io/crates/l/kv-derive)

## Examples

### `#[derive(ToVec)]`

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

### `#[derive(FromIter)]`

```rust
use kv_derive::FromIter;

#[derive(FromIter, Default, Debug, PartialEq)]
struct Foo {
    bar: i32,
    qux: String,
}

let actual = Foo::from_iter(vec![("bar", "42"), ("qux", "quuux")]).unwrap();
let expected = Foo { bar: 42, qux: "quuux".into() };
assert_eq!(actual, expected);
```

`FromIter` requires that the deriving struct implements [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) because some fields may be missing in the iterator.
