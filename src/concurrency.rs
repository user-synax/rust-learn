// concurrency.rs - Threads, channels, shared state
//
// Run: cargo run -- concurrency
// Model: "Share memory by communicating" (channels) > shared Mutex.
// Ownership guarantees no data races at COMPILE time.

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

pub fn run() {
    println!("--- concurrency ---");

    // --- 1. spawn + join ---
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        "done from thread"
    });
    println!("main keeps working...");
    println!("joined: {}", handle.join().unwrap());

    // --- 2. move ownership into thread ---
    let data = vec![1, 2, 3];
    let h = thread::spawn(move || {
        println!("thread owns data: {:?}", data);
    });
    h.join().unwrap();
    // println!("{:?}", data); // ERROR: moved

    // --- 3. Channels (mpsc: multi-producer, single-consumer) ---
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // multiple senders

    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(format!("msg {i}")).unwrap();
        }
    });
    thread::spawn(move || {
        tx2.send("hello from second sender".to_string()).unwrap();
    });

    for msg in rx {
        println!("received: {msg}");
    }

    // --- 4. Shared state: Arc<Mutex<T>> ---
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = c.lock().unwrap(); // lock, auto-unlocks on drop
            *n += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("counter = {}", *counter.lock().unwrap());
}
