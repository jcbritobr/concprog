use std::{sync::atomic::{Ordering::Relaxed, AtomicI32}, thread};

fn main() {
    thread::scope(|s|{
        for i in 0..10 {
            s.spawn(move ||{
                let data = get_x();
                println!("{i}: {data}");
            });
        }
    });
    println!("done!");
}

fn get_x() -> i32 {
    static X: AtomicI32 = AtomicI32::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        println!("generating data ...");
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn calculate_x() -> i32 {
    5
}