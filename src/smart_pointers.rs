// smart_pointers.rs - Box, Rc, RefCell, Arc
//
// Run: cargo run -- smart_pointers
// Use when ownership rules are too strict for your data shape:
// Box = single owner, heap. Rc = multiple owners (single thread).
// RefCell = mutate through shared ref (runtime borrow check).
// Arc = Rc for threads. Mutex/RefCell inside for mutation.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>), // Box = fixed size (recursive type needs indirection)
    Nil,
}

pub fn run() {
    println!("--- smart_pointers ---");

    // --- Box<T>: heap value, single owner ---
    let b = Box::new(42);
    println!("box = {b}");
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("list = {:?}", list);

    // --- Rc<T>: reference-counted, multiple owners (single-threaded) ---
    let shared = Rc::new(vec![1, 2, 3]);
    let owner_a = Rc::clone(&shared);
    let owner_b = Rc::clone(&shared);
    println!(
        "count = {}, a={:?}, b={:?}",
        Rc::strong_count(&shared),
        owner_a,
        owner_b
    );

    // --- RefCell<T>: interior mutability (borrow checked at RUNTIME) ---
    let cell = RefCell::new(5);
    *cell.borrow_mut() += 10;
    println!("cell = {}", cell.borrow());
    // let _a = cell.borrow_mut();
    // let _b = cell.borrow_mut(); // PANICS at runtime (not compile time)

    // --- Rc<RefCell<T>>: shared + mutable (classic graph/tree pattern) ---
    let shared_mut = Rc::new(RefCell::new(0));
    let c1 = Rc::clone(&shared_mut);
    let c2 = Rc::clone(&shared_mut);
    *c1.borrow_mut() += 1;
    *c2.borrow_mut() += 2;
    println!("shared_mut = {}", shared_mut.borrow());

    // --- Arc<T>: thread-safe Rc ---
    let atomic = Arc::new(100);
    let t1 = Arc::clone(&atomic);
    println!("arc = {}, clone = {}", atomic, t1);
    // For mutation across threads: Arc<Mutex<T>> (see concurrency.rs)
}
