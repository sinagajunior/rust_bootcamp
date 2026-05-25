use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Mutex + Arc Shared Counter");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
            println!("Thread {} finished", i);
        });
        handles.push(handle);
    }
    for hadle in handles {
        hadle.join().unwrap();
    }
    println!("Final Counter value: {}", *counter.lock().unwrap());
}
