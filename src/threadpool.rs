use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

struct ThreadPool {
    _buffer: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    _id: usize,
    _handle: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + Sync + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        Self {
            _id: id,
            _handle: handle,
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut buffer = Vec::with_capacity(size);
        for id in 0..size {
            buffer.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            _buffer: buffer,
            sender,
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

fn main() {
    let buffer = Arc::new(Mutex::new(vec![1]));
    let pool = ThreadPool::new(3);

    for _ in 0..10 {
        let buffer = Arc::clone(&buffer);
        pool.execute(move || {
            for i in 0..10 {
                let mut data = buffer.lock().unwrap();
                data.push(i);
            }
        });
    }

    thread::sleep(Duration::from_secs(3));

    let data = Arc::try_unwrap(buffer).unwrap();
    let data = data.into_inner().unwrap();
    println!("{data:?}");
}
