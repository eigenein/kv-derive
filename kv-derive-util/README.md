# `kv-derive-util`

Additional utilities for `kv-derive`.

[![Crates.io](https://img.shields.io/crates/v/kv-derive)](https://crates.io/crates/kv-derive)
[![Last commit](https://img.shields.io/github/last-commit/eigenein/kv-derive)](https://github.com/eigenein/kv-derive/commits/master)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/eigenein/kv-derive/Check)](https://github.com/eigenein/kv-derive/actions)
![License: MIT](https://img.shields.io/crates/l/kv-derive)

# [`kv_derive_util::BooleanAsU8`]

```rust
use kv_derive::prelude::*;
use kv_derive::{FromIter, IntoVec};

#[derive(IntoVec, FromIter, Debug, PartialEq)]
struct Foo {
    #[kv(default(), via = "kv_derive_util::BooleanAsU8")]
    bar: bool,
}

assert_eq!(Foo { bar: true }.into_vec(), vec![("bar".into(), "1".into())]);
```

# [`kv_derive_util::chrono::DateTimeAsTimestamp`]

```rust
use chrono::{DateTime, TimeZone, Utc};
use kv_derive::prelude::*;
use kv_derive::{FromIter, IntoVec};

#[derive(IntoVec, Debug, PartialEq)]
struct Foo {
    #[kv(
        default(value = "Utc::now()"),
        via = "kv_derive_util::chrono::DateTimeAsTimestamp",
    )]
    bar: DateTime<Utc>,
}

assert_eq!(
    Foo { bar: Utc.timestamp(1653058287, 0) }.into_vec(),
    vec![("bar".into(), "1653058287".into())],
);

// assert_eq!(
//     Foo::from_iter(vec![("bar", "1653058287")]).unwrap(),
//     Foo { bar: Utc.timestamp(1653058287, 0) },
// );
```
