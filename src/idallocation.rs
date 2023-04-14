use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

fn main() {
    println!("requesting id ....");
    let id = allocate_new_id();
    println!("requested id: {id}");

    let id = allocate_new_id();
    println!("requested id: {id}");

    let id = allocate_new_id();
    println!("requested id: {id}");

    let id = allocate_new_id_p();
    println!("requested id with panikable function: {}", id);

    let id = allocate_new_id_solution();
    println!("requested id with panikable function: {}", id);
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Relaxed)
}

fn allocate_new_id_p() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    assert!(id < 1000, "too many ids!");
    id
}

fn allocate_new_id_solution() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Relaxed);
        panic!("too many ids!");
    }
    id
}
