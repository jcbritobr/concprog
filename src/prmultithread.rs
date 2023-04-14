/* Progress Reporting from Multiple Thread */

use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
    time::Duration,
};

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            println!("working ..{n}/100 done");
            if n == 100 {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("done");
}

fn process_item(_data: i32) {
    thread::sleep(Duration::from_secs(1));
}
