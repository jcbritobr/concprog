use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

fn main() {
    for _ in 0..1001 {
        let id = allocate_new_id();
        println!("id: {id}");
    }
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1000, "too many IDS");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}
