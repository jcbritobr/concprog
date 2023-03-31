use std::thread;

fn main() {
    println!("running scoped threads examples");
    let numbers = vec![1, 2, 3];

    thread::scope(|s|{
        s.spawn(||{
            println!("length: {}", numbers.len());
        });
        s.spawn(||{
            for i in &numbers{
                println!("{i}");
            }
        });
    });

    for i in &numbers {
        println!(">> {i}");
    }
}