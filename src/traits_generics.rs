// traits_generics.rs - Shared behavior (traits) + generic code
//
// Run: cargo run -- traits_generics
// Think: trait ~= interface, generics ~= templates.

use std::fmt::Display;

// --- Generic function: works for any T that can be compared + displayed ---
fn largest<T: PartialOrd + Display + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// --- Trait definition ---
trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}
struct Robot {
    id: u32,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hi, I'm {}", self.name)
    }
}
impl Greet for Robot {
    fn greet(&self) -> String {
        format!("BEEP. Unit {} online.", self.id)
    }
}

// Static dispatch: monomorphized, fast (preferred)
fn hello_static<T: Greet>(g: &T) {
    println!("{}", g.greet());
}

// Dynamic dispatch: trait object, flexible (heterogeneous list)
fn hello_dynamic(g: &dyn Greet) {
    println!("{}", g.greet());
}

// --- Default method + derive ---
trait Summary {
    fn summarize(&self) -> String {
        " (no summary) ".to_string()
    }
}

#[derive(Debug)]
struct Article {
    title: String,
}
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

pub fn run() {
    println!("--- traits_generics ---");

    println!("largest int: {}", largest(5, 9));
    println!("largest char: {}", largest('a', 'z'));

    let p = Person {
        name: "Ayush".to_string(),
    };
    let r = Robot { id: 7 };
    hello_static(&p);
    hello_static(&r);
    hello_dynamic(&p);
    hello_dynamic(&r);

    // Trait objects in a Vec (different types, same behavior)
    let greeters: Vec<Box<dyn Greet>> = vec![Box::new(p), Box::new(r)];
    for g in &greeters {
        hello_dynamic(g.as_ref());
    }

    let art = Article {
        title: "Rust traits".to_string(),
    };
    println!("{:?} -> {}", art, art.summarize());
}
