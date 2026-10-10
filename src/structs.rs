// structs.rs - Custom data with methods
//
// Run: cargo run -- structs

// Classic struct
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

// Tuple struct
#[derive(Debug)]
struct Point(i32, i32);

// Unit struct (marker, no data)
struct AlwaysActive;

impl User {
    // Associated function (like static method) - constructor pattern
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
            active: true,
        }
    }

    // Method (&self = borrow, no ownership taken)
    fn greet(&self) -> String {
        format!("Hi, I'm {} ({}y)", self.name, self.age)
    }

    // Mutable method
    fn birthday(&mut self) {
        self.age += 1;
    }

    // Consuming method (takes ownership)
    fn deactivate(self) -> Self {
        Self {
            active: false,
            ..self
        }
    }
}

pub fn run() {
    println!("--- structs ---");

    let mut u = User::new("Ayush".to_string(), 20);
    println!("{:?}", u);
    println!("{}", u.greet());

    u.birthday();
    println!("after birthday: {:?}", u);
    println!("active? {}", u.active);

    let u = u.deactivate();
    println!("after deactivate: {:?}", u);

    // Struct update syntax
    let u2 = User {
        name: "Bob".to_string(),
        ..u // copy remaining fields from u (u partially moved!)
    };
    println!("u2 = {:?}", u2);

    let p = Point(3, 4);
    println!("point: {}, {}", p.0, p.1);

    let _marker = AlwaysActive;
}
