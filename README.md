# `kv-derive`

Derive `struct` conversions from and to key-value vectors using [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) and [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html).

[![Crates.io](https://img.shields.io/crates/v/kv-derive)](https://crates.io/crates/kv-derive)
[![Last commit](https://img.shields.io/github/last-commit/eigenein/kv-derive)](https://github.com/eigenein/kv-derive/commits/master)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/eigenein/kv-derive/Check)](https://github.com/eigenein/kv-derive/actions)
![License: MIT](https://img.shields.io/crates/l/kv-derive)

## Examples

### `#[derive(IntoVec)]`

```rust
use kv_derive::prelude::*;

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
use kv_derive::prelude::*;

#[derive(FromIter, Debug, PartialEq)]
struct Foo {
    #[kv(default())]
    bar: i32,
    
    #[kv(default())]
    qux: String,
}

let actual = Foo::from_iter(vec![("bar", "42"), ("qux", "quuux")]).unwrap();
let expected = Foo { bar: 42, qux: "quuux".into() };
assert_eq!(actual, expected);
```

### `#[derive(FromMapping)]`

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;

#[derive(FromMapping, Debug, PartialEq)]
struct Foo {
    bar: i32,
    qux: String,
}

let mapping = HashMap::from([("bar", "42"), ("qux", "quuux")]);
let actual = Foo::from_mapping(&mapping).unwrap();
let expected = Foo { bar: 42, qux: "quuux".into() };
assert_eq!(actual, expected);
```

Missing key causes the error:

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;

#[derive(FromMapping, Debug, PartialEq)]
struct Foo {
    bar: i32,
    qux: String,
}

let mapping = HashMap::from([("bar", "42")]);
let actual = Foo::from_mapping(&mapping);
assert_eq!(actual, Err(kv_derive::Error::MissingKey("qux")));
```

## Customizing fields

### Optional fields with `Option<T>`

`Option<T>` fields are skipped while converting to a vector:

```rust
use kv_derive::prelude::*;

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
use kv_derive::prelude::*;

#[derive(FromIter, Debug, PartialEq)]
struct Foo {
    #[kv(default())]
    bar: Option<i32>,
    
    #[kv(default())]
    qux: Option<i32>,
    
    #[kv(default(value = "Some(42)"))]
    quux: Option<i32>,
}

let actual = Foo::from_iter(vec![("bar", "42")]).unwrap();
let expected = Foo { bar: Some(42), qux: None, quux: Some(42) };
assert_eq!(actual, expected);
```

### Defaults with `kv(default(…))`

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;

#[derive(FromMapping, Debug, PartialEq)]
struct Foo {
    #[kv(default())]
    bar: i32,
    
    #[kv(default(value = "42"))]
    qux: i32,
}

let foo = Foo::from_mapping(&HashMap::<String, String>::new()).unwrap();
assert_eq!(foo, Foo { bar: 0, qux: 42 });
```

### Renaming fields with `kv(rename = …)`

Uses the specified key instead of the identifier:

```rust
use kv_derive::prelude::*;

#[derive(IntoVec)]
struct Foo {
    #[kv(rename = "qux")]
    bar: i32,
}

let foo = Foo { bar: 42 };
assert_eq!(foo.into_vec(), vec![("qux".into(), "42".into())]);
```

### `Vec<T>` fields

Vector field emits multiple entries with the same key:

```rust
use kv_derive::prelude::*;

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
use kv_derive::prelude::*;

#[derive(FromIter, Debug, PartialEq)]
struct Foo {
    #[kv(default())]
    bar: Vec<i32>,
}

let actual = Foo::from_iter(vec![("bar", "42".into()), ("bar", "100500".into())]).unwrap();
let expected = Foo { bar: vec![42, 100500] };
assert_eq!(actual, expected);
```

## Flattening

### Simple

It is possible to «flatten» an inner structure into the resulting `Vec<_>`:

```rust
use kv_derive::prelude::*;

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

Note that the macro doesn't check for possible duplicate keys in outer and inner structures.

It's **not** possible to restore a structure with flattened fields using `#[derive(FromIter)]`.

### Prefixed

It's also possible to prefix all the inner fields with a same prefix:

```rust
use kv_derive::prelude::*;

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
