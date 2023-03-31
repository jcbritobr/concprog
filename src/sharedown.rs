use std::{sync::Arc, thread};

fn main() {
    // leaking threads
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr());

    thread::spawn(move || dbg!(b));

    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });

    dbg!(a);
}
