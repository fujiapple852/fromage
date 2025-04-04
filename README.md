![ci](https://github.com/fujiapple852/fromage/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/fromage/badge.svg)](https://docs.rs/fromage/0.1.0)
[![Crate](https://img.shields.io/crates/v/fromage.svg)](https://crates.io/crates/fromage/0.1.0)

# Fromage 🧀

A cheesy Rust hack for converting between _non-local_ types.

TL;DR: Allows implementing `From` and `TryFrom` like traits for _non-local_ types without violating
the [orphan rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules).

This crate has no dependencies or macros, is `no_std` and forbids `unsafe` code.

## Example

Convert between the two non-local types `String` and `usize`:

```rust
use fromage::{Fromage, TryFromage};

struct X;
impl Fromage<String, X> for usize {
    fn fromage(value: String) -> Self {
        value.len()
    }
}
impl TryFromage<String, X> for usize {
    type Error = ();

    fn try_fromage(value: String) -> Result<Self, Self::Error> {
        Ok(value.len())
    }
}

#[test]
fn test() {
    assert_eq!(5_usize, usize::fromage(String::from("hello")));
    assert_eq!(5_usize, usize::try_fromage(String::from("world")).unwrap());
}
```

## Status

Experimental.

## How it works

The [orphan rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules) state that:

```
> Given impl<P1..=Pn> Trait<T1..=Tn> for T0, an impl is valid only if at least one of the following is true:
>
> - `Trait` is a local trait
> - All of:
>   - At least one of the types T0..=Tn must be a local type. Let Ti be the first such type.
>   - No uncovered type parameters P1..=Pn may appear in T0..Ti (excluding Ti)
```

Fromage defines the `Fromage` and `TryFromage` traits that mirror the traits in the standard library, except that they
require an `impl` defined additional type parameter `X` and therefore fulfil the _"At least one of the types `T0..=Tn`
must be a local type"_ clause above.

The type parameter `X` is not used in the trait methods. It may be
any type provided it is both [local](https://doc.rust-lang.org/reference/glossary.html#local-type)
and [uncovered](https://doc.rust-lang.org/reference/glossary.html#uncovered-type), typically
a [unit struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields). It
may be reused for all `Fromage` and `TryFromage` implementations within a crate.

The `Fromage` and `TryFromage` traits define methods named `fromage` and `try_fromage` respectively to avoid conflicting
with the standard library's `From` and `TryFrom` traits.

## Limitations

The `Fromage` and `TryFromage` traits defined in this crate are distinct from the standard library's `From` and
`TryFrom` traits and are not interchangeable. Therefore, if a crate uses the standard library's `From` and `TryFrom`
traits in its public API, you cannot use the `Fromage` and `TryFromage` traits to implement conversions for the
types required by crate.

For example, if a crate exposes the following public API then you cannot use an `impl Fromage<Foo> for Bar` to
implement the conversion.

```rust
pub fn foo(value: impl Into<Bar>) { ... }
```

## Alternatives

### Newtype

Typically, a
local [newtype](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-typesl)
is used to implement the standard library's `From` and `TryFrom` traits when both types are non-local.

Either:

```rust
struct FooWrapper(Foo);

impl From<FooWrapper> for Bar {
    fn from(_value: FooWrapper) -> Self {
        Bar
    }
}
```

Or:

```rust
struct BarWrapper(Bar);

impl From<Foo> for BarWrapper {
    fn from(_value: Foo) -> Self {
        BarWrapper(Bar)
    }
}
```

### Conversion function

Simple conversions functions can be used instead of the `From` and `TryFrom` traits.

```rust
fn convert(_value: Foo) -> Bar {
    Bar
}
fn try_convert(_value: Foo) -> Result<Bar, ()> {
    Ok(Bar)
}
```

### Local traits

For completeness, the orphan rules allow implementing local `MyFrom` and `MyTryFrom` traits which may then be used to
with non-local types.

This is not recommended as it requires a significant amount of boilerplate code whilst sharing the same limitations as
the Fromage approach.

## License

Fromage is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2025