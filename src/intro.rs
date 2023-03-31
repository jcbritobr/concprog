use std::thread;

fn main() {
    println!("Hello, world!");
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    t1.join().unwrap();
    t2.join().unwrap();

    println!("hello from main thread");
    let buffer = vec![1, 2, 3];
    thread::spawn(move || {
        for i in buffer {
            println!(">> item >> {}", i);
        }
    })
    .join()
    .unwrap();

    let numbers = Vec::from_iter(1..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("averate: {average}");
}

fn f() {
    println!("hello from another thread!");
    let id = thread::current().id();
    println!("this is my thread id: {id:?}");
}
