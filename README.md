# `kv-derive`

Derive `struct` conversions from and to string key-value vectors using [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) and [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html).

[![Crates.io](https://img.shields.io/crates/v/kv-derive)](https://crates.io/crates/kv-derive)
[![Last commit](https://img.shields.io/github/last-commit/eigenein/kv-derive)](https://github.com/eigenein/kv-derive/commits/master)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/eigenein/kv-derive/Check)](https://github.com/eigenein/kv-derive/actions)
![License: MIT](https://img.shields.io/crates/l/kv-derive)

## Examples

Any type that implements [`std::string::ToString`] and/or [`std::str::FromStr`] supported as a field type:

### `#[derive(IntoVec)]`

```rust
use kv_derive::prelude::*;
use kv_derive::IntoVec;

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
use kv_derive::FromIter;

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

`#[derive(FromIter)]` requires that you specify `#[kv(default(…))]` attribute on each field, because it needs to know what to do when the key is missing in the input.

### `#[derive(FromMapping)]`

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;
use kv_derive::FromMapping;

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

Here `#[kv(default(…))]` is not required, and missing key causes the error:

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;
use kv_derive::FromMapping;

#[derive(FromMapping, Debug, PartialEq)]
struct Foo {
    bar: i32,
    qux: String,
}

let mapping = HashMap::from([("bar", "42")]);
let actual = Foo::from_mapping(&mapping);
assert_eq!(actual, Err(kv_derive::error::Error::MissingKey("qux")));
```

## Customizing fields

### Optional fields

With `#[kv(optional)]` the macro expects that the fields are wrapped with [`std::option::Option`], and skips `None` values:

```rust
use kv_derive::prelude::*;
use kv_derive::IntoVec;

#[derive(IntoVec)]
struct Foo {
    #[kv(optional)]
    bar: Option<i32>,
    
    #[kv(optional)]
    qux: Option<i32>,
}

let foo = Foo { bar: Some(42), qux: None };
assert_eq!(foo.into_vec(), vec![("bar".into(), "42".into())]);
```

Note that the **both** `#[kv(optional)]` and [`std::option::Option`] type are needed here, because technically you could omit `#[kv(optional)]` and implement [`std::string::ToString`] on a custom `Option<T>` to handle `None` values manually.

For `#[derive(FromIter)]` this also ensures that [`std::str::FromStr`] is called on `T` and not on `Option<T>`:

```rust
use kv_derive::prelude::*;
use kv_derive::FromIter;

#[derive(FromIter, Debug, PartialEq)]
struct Foo {
    #[kv(default(), optional)]
    bar: Option<i32>,
    
    #[kv(default(), optional)]
    qux: Option<i32>,
    
    #[kv(default(value = "Some(42)"), optional)]
    quux: Option<i32>,
}

let actual = Foo::from_iter(vec![("bar", "42")]).unwrap();
let expected = Foo { bar: Some(42), qux: None, quux: Some(42) };
assert_eq!(actual, expected);
```

### Default values

`#[kv(default())]` implies that the type implements [`std::default::Default`]. But you can also specify a custom default value with #[kv(default(value = "<expression>"))]:

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;
use kv_derive::FromMapping;

#[derive(FromMapping, Debug, PartialEq)]
struct Foo {
    #[kv(default())]
    bar: i32,
    
    #[kv(default(value = "42"))]
    qux: i32,
    
    #[kv(default(), optional)]
    quux: Option<i32>,
    
    #[kv(default(value = "Some(100500)"), optional)]
    quuux: Option<i32>,
}

let foo = Foo::from_mapping(&HashMap::<String, String>::new()).unwrap();
assert_eq!(foo, Foo { bar: 0, qux: 42, quux: None, quuux: Some(100500) });
```

### Renaming fields with `#[kv(rename = …)]`

Uses the specified key instead of the identifier:

```rust
use kv_derive::prelude::*;
use kv_derive::IntoVec;

#[derive(IntoVec)]
struct Foo {
    #[kv(rename = "qux")]
    bar: i32,
}

let foo = Foo { bar: 42 };
assert_eq!(foo.into_vec(), vec![("qux".into(), "42".into())]);
```

### Convert to and from another representation

Here's an example how you could represent a boolean value with an `i32`:

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;
use kv_derive::{IntoVec, FromIter, FromMapping};

#[derive(IntoVec, FromIter, FromMapping, PartialEq, Debug)]
struct Foo {
    #[kv(
        default(),
        collection,
        into_repr_with = "|value| value as i32",
        from_repr_with = "|value: i32| kv_derive::result::Result::Ok(value != 0)",
    )]
    bar: Vec<bool>,
}

assert_eq!(
    Foo { bar: vec![false, true] }.into_vec(),
    vec![("bar".into(), "0".into()), ("bar".into(), "1".into())],
);
assert_eq!(
    Foo::from_iter(vec![("bar".into(), "0".into()), ("bar".into(), "1".into())]).unwrap(), 
    Foo { bar: vec![false, true] },
);
assert_eq!(
    Foo::from_mapping(HashMap::from([("bar", "1")])).unwrap(),
    Foo { bar: vec![true] },
);
```

In this case, [`std::string::ToString`] and [`std::str::FromStr`] operate on `i32` rather than `bool`.

### Collection fields

```rust
use kv_derive::prelude::*;
use kv_derive::IntoVec;

#[derive(IntoVec)]
struct Foo {
    #[kv(collection)]
    bar: Vec<i32>,
}

let foo = Foo { bar: vec![42, 100500] };
assert_eq!(foo.into_vec(), vec![
    ("bar".into(), "42".into()),
    ("bar".into(), "100500".into()),
]);
```

```rust
use kv_derive::prelude::*;
use kv_derive::FromIter;

#[derive(FromIter, Debug, PartialEq)]
struct Foo {
    #[kv(collection, default())]
    bar: Vec<i32>,
}

let actual = Foo::from_iter(vec![("bar", "42".into()), ("bar", "100500".into())]).unwrap();
let expected = Foo { bar: vec![42, 100500] };
assert_eq!(actual, expected);
```

#### Note for `#[derive(FromMapping)]`

`HashMap` or `BTreeMap` cannot contain duplicate keys. However, for consistency, singular values are properly converted to [`std::vec::Vec`]s:

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;
use kv_derive::FromMapping;

#[derive(FromMapping, Debug, PartialEq)]
struct Foo {
    #[kv(collection)]
    bar: Vec<i32>,
}

let map = HashMap::from([("bar", "42")]);
let actual = Foo::from_mapping(&map).unwrap();
let expected = Foo { bar: vec![42] };
assert_eq!(actual, expected);
```

## Flattening

### Simple flattening

It is possible to «flatten» an inner structure:

```rust
use kv_derive::prelude::*;
use kv_derive::IntoVec;

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

It's **not** possible to derive [`FromIter`](crate::prelude::FromIter) for a structure with a flattened field. However, it works for `#[derive(FromMapping)]`:

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;
use kv_derive::FromMapping;

#[derive(FromMapping, Debug, PartialEq)]
struct Inner {
    bar: i32,
}

#[derive(FromMapping, Debug, PartialEq)]
struct Outer {
    #[kv(flatten())]
    inner: Inner,
}

let map = HashMap::from([("bar", "42")]);
let actual = Outer::from_mapping(&map).unwrap();
let expected = Outer { inner: Inner { bar: 42 } };
assert_eq!(actual, expected);
```

### Prefixed flattening

It's also possible to prefix all the inner fields with a same prefix:

```rust
use kv_derive::prelude::*;
use kv_derive::IntoVec;

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

And back:

```rust
use std::collections::HashMap;

use kv_derive::prelude::*;
use kv_derive::FromMapping;

#[derive(FromMapping, Debug, PartialEq)]
struct Inner {
    bar: i32,
}

#[derive(FromMapping, Debug, PartialEq)]
struct Outer {
    #[kv(flatten(prefix = "inner::"))]
    inner: Inner,
}

let map = HashMap::from([("inner::bar", "42")]);
let actual = Outer::from_mapping(&map).unwrap();
let expected = Outer { inner: Inner { bar: 42 } };
assert_eq!(actual, expected);
```
