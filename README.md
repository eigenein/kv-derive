# `kv-derive`

Derive `struct` conversions from and to key-value vectors using [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) and [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html).

[![Crates.io](https://img.shields.io/crates/v/kv-derive)](https://crates.io/crates/kv-derive)
[![Last commit](https://img.shields.io/github/last-commit/eigenein/kv-derive)](https://github.com/eigenein/kv-derive/commits/master)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/eigenein/kv-derive/Check)](https://github.com/eigenein/kv-derive/actions)
![License: MIT](https://img.shields.io/crates/l/kv-derive)

## Examples

### `#[derive(IntoVec)]`

```rust
use kv_derive::IntoVec;
use kv_derive_impl::IntoVec;

#[derive(IntoVec)]
struct Foo {
    bar: i32,
    qux: String,
}

let foo = Foo { bar: 42, qux: "qux".into() };
assert_eq!(foo.into_vec(), vec![
    ("bar".into(), "42".into()),
    ("qux".into(), "qux".into()),
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

let actual = Foo::from_iter(vec![("bar", "42"), ("qux", "quuux")])?;
let expected = Foo { bar: 42, qux: "quuux".into() };
assert_eq!(actual, expected);

# ::kv_derive_impl::Result::Ok(())
```

`FromIter` requires that the deriving struct implements [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html).

## Customizing fields

### Optional fields

`Option<T>` fields are skipped while converting to a vector:

```rust
use kv_derive::IntoVec;
use kv_derive_impl::IntoVec;

#[derive(IntoVec)]
struct Foo {
    bar: Option<i32>,
    qux: Option<i32>,
}

let foo = Foo { bar: Some(42), qux: None };
assert_eq!(foo.into_vec(), vec![("bar".into(), "42".into())]);
```

and left out with their defaults while converting back to the struct:

```rust
use kv_derive::FromIter;

#[derive(FromIter, Default, Debug, PartialEq)]
struct Foo {
    bar: Option<i32>,
    qux: Option<i32>,
}

let actual = Foo::from_iter(vec![("bar", "42")])?;
let expected = Foo { bar: Some(42), qux: None };
assert_eq!(actual, expected);

# ::kv_derive_impl::Result::Ok(())
```

### Collection fields

Collection field emits multiple entries with the same key:

```rust
use kv_derive::IntoVec;
use kv_derive_impl::IntoVec;

#[derive(IntoVec)]
struct Foo {
    bar: Vec<i32>,
}

let foo = Foo { bar: vec![42, 100500] };
assert_eq!(foo.into_vec(), vec![
    ("bar".into(), "42".into()),
    ("bar".into(), "100500".into()),
]);
```

which can be recollected back:

```rust
use kv_derive::FromIter;

#[derive(FromIter, Default, Debug, PartialEq)]
struct Foo {
    bar: Vec<i32>,
}

let actual = Foo::from_iter(vec![("bar", "42".into()), ("bar", "100500".into())])?;
let expected = Foo { bar: vec![42, 100500] };
assert_eq!(actual, expected);

# ::kv_derive_impl::Result::Ok(())
```

### Renaming fields with `kv(rename = …)`

Uses the specified key instead of the identifier:

```rust
use kv_derive::IntoVec;
use kv_derive_impl::IntoVec;

#[derive(IntoVec)]
struct Foo {
    #[kv(rename = "qux")]
    bar: i32,
}

let foo = Foo { bar: 42 };
assert_eq!(foo.into_vec(), vec![("qux".into(), "42".into())]);
```

## Flattening

### Simple

```rust
use kv_derive::IntoVec;
use kv_derive_impl::IntoVec;

#[derive(IntoVec)]
struct Bar {
    qux: i32,
}

#[derive(IntoVec)]
struct Foo {
    #[kv(flatten())]
    bar: Bar,
}

let foo = Foo { bar: Bar { qux: 42 } };
assert_eq!(foo.into_vec(), vec![("qux".into(), "42".into())]);
```

### Prefixed

```rust
use kv_derive::IntoVec;
use kv_derive_impl::IntoVec;

#[derive(IntoVec)]
struct Bar {
    qux: i32,
}

#[derive(IntoVec)]
struct Foo {
    #[kv(flatten(prefix = "bar::"))]
    bar: Bar,
}

let foo = Foo { bar: Bar { qux: 42 } };
assert_eq!(foo.into_vec(), vec![("bar::qux".into(), "42".into())]);
```
