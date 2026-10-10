// enums.rs - Enums + Option + Result + match
//
// Run: cargo run -- enums
// Enums in Rust hold DATA, not just labels (like TypeScript unions).

#[derive(Debug)]
enum Role {
    Admin,
    User(String),          // data attached
    Guest { visits: u32 }, // struct-like variant
}

#[derive(Debug)]
#[allow(dead_code)]
enum Payment {
    Cash(u32),
    Card(String, u32),
}

#[allow(clippy::unnecessary_literal_unwrap)]
pub fn run() {
    println!("--- enums ---");

    let r1 = Role::Admin;
    let r2 = Role::User("ayush".to_string());
    let r3 = Role::Guest { visits: 3 };
    println!("{:?} {:?} {:?}", r1, r2, r3);

    // Match must be exhaustive
    for role in [r1, r2, r3] {
        match role {
            Role::Admin => println!("admin: full access"),
            Role::User(name) => println!("user: {name}"),
            Role::Guest { visits } => println!("guest, visits={visits}"),
        }
    }

    // --- Option<T>: value may be absent (NOT null) ---
    let some: Option<i32> = Some(10);
    let none: Option<i32> = None;
    println!("some={:?}, none={:?}", some, none);
    println!("unwrap_or: {}", none.unwrap_or(0));
    println!("map: {:?}", some.map(|v| v * 2));

    // --- Result<T, E>: operation may fail ---
    match divide(10.0, 2.0) {
        Ok(v) => println!("10/2 = {v}"),
        Err(e) => println!("error: {e}"),
    }
    match divide(10.0, 0.0) {
        Ok(v) => println!("10/0 = {v}"),
        Err(e) => println!("error: {e}"),
    }

    let p = Payment::Card("4111".to_string(), 500);
    println!("payment: {:?}", p);
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
