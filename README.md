# Rust Transmute without Unsafe!

This crate allows you to convert any type to any other type without the use of unsafe.

It provides [one function](src/lib.rs), implemented entirely in Safe Rust:

```
fn transmute<T, U>(x: T) -> U
```

Usable in code that is marked `#![forbid(unsafe_code)]`.

Never fight the borrow checker again!

All versions of Rust since 1.0 are supported.

**Examples**:

Sometimes you want to dereference a null pointer. This crate allows you to do
so without any unsafe code.

```rust
let p = core::ptr::null_mut();
let x = transmute::<*mut i64, &'static i64>(p);
println!("{}", x); // BOOM
```

---

Sometimes you want to grow an array, but don't want to allocate more space.
Thanks to modern technology, this is now possible to do safely™. Just steal
the nearby memory with `transmute`.

```rust
let p = [0];
let mut x = transmute::<[u64; 1], [u64; 100]>(p);
// this slice now has 100 elements: yay so much space for our bytes
assert!(x.len() == 100);
x[50] = 42;
```

# ⚠️⚠️⚠️ WARNING ⚠️⚠️⚠️

THIS CRATE IS NOT SAFE.

This crate abuses a bug in the Rust compiler
(https://github.com/rust-lang/rust/issues/57893) to circumvent the Rust type
system.

Please do not actually use this.

**Moral of the story**: you cannot rely solely on Rust for supply chain security.
