use std::{
    sync::atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for i in 0..4 {
            s.spawn(move || {
                for t in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }
        // the main thread shows status updates, every second
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);

            if n == 0 {
                println!("working... nothing done yet")
            } else {
                println!(
                    "working ...{n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time,
                );
            }
            if n == 100 {
                break;
            }
            thread::sleep(Duration::from_secs(1))
        }
    });
    println!("done!");
}

fn process_item(_data: i32) {
    thread::sleep(Duration::from_secs(1));
}
