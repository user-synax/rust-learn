// data_types.rs - Scalar + Compound types, String vs &str
//
// Run: cargo run -- data_types

pub fn run() {
    println!("--- data_types ---");

    // --- Integers ---
    let i8_val: i8 = -128;
    let u8_val: u8 = 255;
    let default_int = 42; // i32 by default
    println!("ints: {i8_val}, {u8_val}, {default_int}");

    // --- Floats ---
    let f64_val = 2.0; // f64 default
    let f32_val: f32 = 2.5;
    println!("floats: {f64_val}, {f32_val}");

    // --- Bool + char (4 bytes, Unicode) ---
    let active: bool = true;
    let emoji: char = '🦀';
    println!("bool={active}, char={emoji}");

    // --- Tuple (fixed size, mixed types) ---
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    println!("tuple: {}, {}, {}", tup.0, tup.1, tup.2);
    let (t1, t2, t3) = tup; // destructure
    println!("destructured: {t1}, {t2}, {t3}");

    // --- Array (fixed length, same type, on stack) ---
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3]; // [0, 0, 0]
    println!("arr[0]={}, zeros={:?}", arr[0], zeros);

    // --- Slice (view into array/vec) ---
    let slice = &arr[1..4];
    println!("slice = {:?}", slice);

    // --- String vs &str (MOST IMPORTANT for real projects) ---
    // &str = borrowed string slice (view, usually literal)
    // String = owned, growable, heap-allocated
    let s_literal: &str = "hello"; // lives in binary, borrowed
    let mut s_owned: String = String::from("hello");
    s_owned.push_str(" world");
    s_owned.push('!');
    println!("&str={s_literal}, String={s_owned}");

    // Convert between them
    let to_owned: String = s_literal.to_string();
    let to_slice: &str = &s_owned; // deref to &str
    println!("converted: {to_owned}, {to_slice}");

    // --- Type conversion (explicit, no implicit casts) ---
    let n: i32 = 42;
    let as_i64 = n as i64;
    let parsed: i32 = "100".parse().expect("parse failed");
    println!("cast={as_i64}, parsed={parsed}");
}
