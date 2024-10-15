# Rust Transmute without Unsafe!

This crate allows you to convert any type to any other type without the use of unsafe.

It provides one function:

```
fn transmute<T, U>(x: T) -> U
```

Usable in code that is marked `#![forbid(unsafe_code)]`.

No need to fight the borrow checker anymore!

# ⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️

THIS CRATE IS NOT SAFE.

This crate abuses a bug in the Rust compiler
(https://github.com/rust-lang/rust/issues/57893) to circumvent the Rust type
system.

Please do not actually use this.

**Moral of the story**: you cannot rely solely on Rust for supply chain security.
