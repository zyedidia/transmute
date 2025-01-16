#![forbid(unsafe_code)]

trait Sup<T> {
    type Assoc;
}

impl<T> Sup<T> for () {
    type Assoc = T;
}
impl<T, U> Dyn<T, U> for () {}

trait Dyn<A, B>: Sup<A, Assoc = A> + Sup<B, Assoc = B> {}

trait Trait<X, Y> {
    type Assoc;
}
impl<X, Y> Trait<X, Y> for dyn Dyn<(), ()> {
    type Assoc = Y;
}
impl<A, B, X, Y> Trait<X, Y> for dyn Dyn<A, B> {
    type Assoc = X;
}

fn transmute_obj<A, B, X, Y>(x: X) -> <dyn Dyn<A, B> as Trait<X, Y>>::Assoc {
    x
}

pub fn transmute<T, U>(x: T) -> U {
    transmute_obj::<(), (), T, U>(x)
}

// This is the old transmute from before Rust's next-generation trait solver
// (https://rust-lang.github.io/rust-project-goals/2024h2/next-solver.html). The next-generation
// trait solver is not yet released, but is available on nightly with -Znext-solver. I'm not sure
// when it is expected to be stabilized. The project began in EOY 2022.
//
// trait Object<U> {
//     type Output;
// }
//
// impl<T: ?Sized, U> Object<U> for T {
//     type Output = U;
// }
//
// fn transmute_obj<T: ?Sized, U>(x: <T as Object<U>>::Output) -> U {
//     x
// }
//
// pub fn transmute<T, U>(x: T) -> U {
//     transmute_obj::<dyn Object<U, Output = T>, U>(x)
// }

#[test]
fn null_pointer() {
    // make a null pointer
    let p = core::ptr::null_mut();
    // "safely" transmute it into a reference
    let _x = transmute::<*mut i64, &'static i64>(p);

    // If we access 'x' we get a segfault: OOPS
    // assert!(*_x == 0);
}

#[test]
fn memory_corruption() {
    let p = [0, 1, 2];
    assert!(p.len() == 3);

    let mut x = transmute::<[u64; 3], [u64; 100]>(p);
    // this slice is now 100 elements: hmmmmm
    assert!(x.len() == 100);
    x[10] = 0xdeadbeef;
}
