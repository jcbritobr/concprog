use std::{
    sync::atomic::{AtomicBool, Ordering::Relaxed},
    thread,
};

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    // Use the main thread to listen user inputs
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    // inform the background thread it needs to stop
    STOP.store(true, Relaxed);
    background_thread.join().unwrap();
}

fn some_work() {
    let _result = 1 + 3;
}
