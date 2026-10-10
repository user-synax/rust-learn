// variables.rs - Variables, Mutability, Constants, Shadowing
//
// Run: cargo run -- variables
// Key ideas:
// - `let` is immutable by default (Rust's safe default)
// - `mut` opts into mutation
// - `const` = compile-time constant, always type-annotated, UPPER_SNAKE
// - `static` = fixed memory location, lives for whole program
// - Shadowing = re-declare with `let` to change type/value (not same as `mut`)

pub const MAX_CONNECTIONS: u32 = 100;
pub static APP_NAME: &str = "learn-rust";

pub fn run() {
    println!("--- variables ---");

    // 1. Immutable by default
    let x = 5;
    println!("x = {x}");
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // 2. Mutable
    let mut y = 5;
    y += 1;
    println!("mut y = {y}");

    // 3. Type annotation (rarely needed, compiler infers)
    let port: u16 = 8080;
    let pi: f64 = std::f64::consts::PI;
    println!("port={port}, pi={pi}");

    // 4. Shadowing (new variable, can change type)
    let s = "  hello  ";
    println!("before shadow: '{s}'");
    let s = s.trim(); // &str, shadowed
    let s = s.len(); // usize, shadowed again
    println!("after shadow: len = {s}");

    // mut vs shadowing:
    let mut count = 1;
    println!("count start = {count}");
    count = 2; // same variable, same type only
    println!("mut count = {count}");
    let count = "two"; // shadowing: new variable, new type allowed
    println!("shadowed count = {count}");

    // 5. Constants vs statics
    println!("MAX_CONNECTIONS = {MAX_CONNECTIONS}, APP_NAME = {APP_NAME}");

    // 6. Scope + shadowing inside blocks
    let n = 10;
    {
        let n = n * 2; // shadows outer n only inside this block
        println!("inner n = {n}");
    }
    println!("outer n = {n}");

    // 7. Destructuring
    let (a, b) = (1, 2);
    println!("destructured: a={a}, b={b}");

    // 8. Unused variables: prefix with _
    let _unused = 42;
}
