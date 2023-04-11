use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();
    
    thread::scope(|s|{
        s.spawn(||{
            for i in 0..100 {
                process_item(i);
                num_done.store((i + 1) as usize, std::sync::atomic::Ordering::Relaxed);
                main_thread.unpark();
            }
        });
        
        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 {break;}
            println!("working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));      
        }
    });
    
    println!("done!");
}

fn process_item(val: i32) {
    let _result = val + 1;
    thread::sleep(Duration::from_secs(1));
}