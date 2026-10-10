// ownership.rs - THE core Rust idea. Read this twice.
//
// Run: cargo run -- ownership
// Rules:
// 1. Each value has ONE owner.
// 2. When owner goes out of scope, value is dropped (freed).
// 3. Move = transfer ownership. Borrow = temporarily use via &.

pub fn run() {
    println!("--- ownership ---");

    // --- Move ---
    let s1 = String::from("hello");
    let s2 = s1; // s1 MOVED to s2. s1 is now invalid.
    // println!("{s1}"); // ERROR: value borrowed after move
    println!("s2 = {s2}");

    // --- Clone (deep copy, explicit + expensive) ---
    let s3 = s2.clone();
    println!("s2={s2}, s3={s3}");

    // --- Copy types (stack-only, no move) ---
    // i32, f64, bool, char, tuples/arrays of Copy types
    let a = 5;
    let b = a; // copies, a still valid
    println!("a={a}, b={b}");

    // --- Borrowing: immutable refs (many allowed) ---
    let s = String::from("rust");
    let len = calc_len(&s); // borrow, s still owned here
    println!("'{s}' len = {len}");

    // --- Mutable borrow (only ONE at a time) ---
    let mut s = String::from("hi");
    change(&mut s);
    println!("changed: {s}");

    // let r1 = &mut s;
    // let r2 = &mut s; // ERROR: cannot borrow as mutable more than once
    // println!("{r1} {r2}");

    // Immutable + mutable cannot coexist:
    // let r1 = &s;
    // let r2 = &mut s; // ERROR
    // println!("{r1}");

    // --- Slices (borrowed views, no ownership) ---
    let word = first_word(&s);
    println!("first word of '{s}' = '{word}'");

    // --- Ownership + functions ---
    let owned = String::from("take me");
    take_ownership(owned); // moved in, can't use after
    // println!("{owned}"); // ERROR

    let v = 10;
    make_copy(v); // Copy, still valid
    println!("v still valid: {v}");

    let s = give_ownership(); // function gives ownership to us
    let s = take_and_give_back(s); // moved in and back out
    println!("back: {s}");
}

fn calc_len(s: &str) -> usize {
    s.len()
} // s (borrow) goes out of scope, nothing freed

fn change(s: &mut String) {
    s.push_str(" there");
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn take_ownership(s: String) {
    println!("took: {s}");
} // s dropped here

fn make_copy(n: i32) {
    println!("copied: {n}");
}

fn give_ownership() -> String {
    String::from("yours")
}

fn take_and_give_back(s: String) -> String {
    s
}
