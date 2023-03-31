use std::cell::{Cell, RefCell};

fn main() {
    println!("it works");
    let buffer = Cell::new(vec![1, 2, 3]);
    mutate(&buffer);
    dbg!(buffer.take());

    let buffer = RefCell::new(vec![1, 2, 3]);
    ref_mutate(&buffer);
    dbg!(buffer);
}

fn mutate(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take();
    v2.push(1);
    v.set(v2);
}

fn ref_mutate(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}
