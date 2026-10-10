// functions.rs - Functions, expressions, return values
//
// Run: cargo run -- functions
// Rule: Rust is expression-based. No `return` needed for last expression (no semicolon).

pub fn run() {
    println!("--- functions ---");

    println!("add(2, 3) = {}", add(2, 3));
    println!("greet = {}", greet("Ayush"));
    println!("max(10, 20) = {}", max(10, 20));

    // Function pointer
    let op: fn(i32, i32) -> i32 = add;
    println!("via fn pointer: {}", op(5, 5));

    // Diverging function (!) demo: uncomment to see
    // fail("boom");
}

// snake_case, typed params, `->` return type
fn add(a: i32, b: i32) -> i32 {
    a + b // expression = return (no semicolon!)
}

fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

// Early return with `return`
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    b
}

// `!` = never returns (panics, loops forever, exits)
#[allow(dead_code)]
fn fail(msg: &str) -> ! {
    panic!("failed: {msg}");
}
