// lifetimes.rs - How long references are valid
//
// Run: cargo run -- lifetimes
// You only need annotations when a function takes + returns references
// and the compiler can't tell which input the output comes from.

pub fn run() {
    println!("--- lifetimes ---");

    let s1 = String::from("hello world");
    let s2 = String::from("hi");
    let longest = longest_str(&s1, &s2);
    println!("longest of '{s1}' / '{s2}' = '{longest}'");

    // Struct holding a reference needs a lifetime
    let text = String::from("name=ayush;role=dev");
    let excerpt = Excerpt {
        part: text.split(';').next().unwrap(),
    };
    println!("excerpt: {}", excerpt.part());
}

// 'a = "both inputs and output live at least as long as 'a"
fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Struct with borrowed data: must declare lifetime
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn part(&self) -> &'a str {
        self.part
    }
}

// 'static = lives for entire program (string literals, leaked memory)
fn _static_example() -> &'static str {
    "I live forever"
}
