// error_handling.rs - panic!, Option/Result, ?, unwrap
//
// Run: cargo run -- error_handling
// Rule of thumb:
// - panic! = bug / impossible case
// - Result = expected failure (IO, network, parse) -> propagate with `?`
// - unwrap/expect = only in examples, tests, or truly impossible cases

use std::fs::File;
use std::io::Read;

#[allow(clippy::unnecessary_literal_unwrap)]
pub fn run() {
    println!("--- error_handling ---");

    // 1. panic! (crashes, use sparingly)
    // panic!("something unrecoverable");

    // 2. Option handling without unwrap
    let name: Option<String> = Some("Ayush".to_string());
    if let Some(n) = name {
        println!("hello {n}");
    }

    // combinators instead of match
    let n: Option<i32> = Some(5);
    let doubled = n.map(|v| v * 2).unwrap_or(0);
    println!("doubled = {doubled}");

    // 3. Result + `?` operator (see read_file below)
    match read_username_len("Cargo.toml") {
        Ok(len) => println!("Cargo.toml len = {len}"),
        Err(e) => println!("read failed: {e}"),
    }

    // 4. unwrap vs expect (intentional demo on known-Ok values)
    let v: Result<i32, &str> = Ok(42);
    println!("unwrap = {}", v.unwrap()); // panics on Err
    let v2: Result<i32, &str> = Ok(7);
    println!("expect = {}", v2.expect("must be Ok")); // panic msg if Err

    // 5. Custom error with match
    match parse_age("25") {
        Ok(age) => println!("age = {age}"),
        Err(e) => println!("parse error: {e}"),
    }
    match parse_age("abc") {
        Ok(age) => println!("age = {age}"),
        Err(e) => println!("parse error: {e}"),
    }

    // 6. ok_or / question-mark in Option-returning fn
    println!("first even: {:?}", first_even(&[1, 3, 4, 7]));
    println!("first even: {:?}", first_even(&[1, 3, 7]));
}

// `?` = if Err, return early with that error. Requires same error type (or From).
fn read_username_len(path: &str) -> Result<usize, std::io::Error> {
    let mut file = File::open(path)?; // propagate open errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // propagate read errors
    Ok(contents.len())
}

fn parse_age(s: &str) -> Result<u8, String> {
    s.parse::<u8>()
        .map_err(|e| format!("'{s}' is not a valid age: {e}"))
}

fn first_even(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().find(|n| n % 2 == 0)
}
