#![forbid(unsafe_code)]

trait Object<U> {
    type Output;
}

impl<T: ?Sized, U> Object<U> for T {
    type Output = U;
}

fn transmute_obj<T: ?Sized, U>(x: <T as Object<U>>::Output) -> U {
    x
}

pub fn transmute<T, U>(x: T) -> U {
    transmute_obj::<dyn Object<U, Output = T>, U>(x)
}

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
