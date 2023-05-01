use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread,
};

struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

fn main() {
    let chann = Arc::new(Channel::<i32>::new());

    let c = Arc::clone(&chann);
    thread::spawn(move || {
        c.send(1);
    });

    let data = chann.receive();
    println!("result: {data}");
}
