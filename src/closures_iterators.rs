// closures_iterators.rs - Closures + powerful iterator chains
//
// Run: cargo run -- closures_iterators
// Closure = anonymous fn that can capture its environment: |args| body
// FnOnce / FnMut / Fn = takes ownership / mut-borrows / borrows env.

pub fn run() {
    println!("--- closures_iterators ---");

    // Basic closure
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // Capturing environment (immutable borrow -> Fn)
    let suffix = "!";
    let shout = |s: &str| format!("{s}{suffix}");
    println!("{}", shout("hello"));

    // Mutable capture (-> FnMut)
    let mut count = 0;
    let mut inc = || {
        count += 1;
        count
    };
    inc();
    inc();
    println!("count = {count}");

    // Ownership capture (move -> FnOnce)
    let data = vec![1, 2, 3];
    let consume = move || {
        println!("consumed: {:?}", data);
    };
    consume();
    // println!("{:?}", data); // ERROR: moved

    // --- Iterator pipeline (the real-world bread and butter) ---
    #[allow(clippy::useless_vec)]
    let users = vec![
        ("alice", 25, true),
        ("bob", 17, true),
        ("cara", 30, false),
        ("dev", 22, true),
    ];

    let adults: Vec<String> = users
        .iter()
        .filter(|(_, age, active)| *age >= 18 && *active) // keep
        .map(|(name, _, _)| name.to_uppercase()) // transform
        .collect(); // run pipeline
    println!("active adults: {:?}", adults);

    // find / any / all / fold
    #[allow(clippy::useless_vec)]
    let nums = vec![2, 4, 6, 7];
    println!("first odd: {:?}", nums.iter().find(|n| *n % 2 == 1));
    println!("any > 5: {}", nums.iter().any(|n| *n > 5));
    println!("all even: {}", nums.iter().all(|n| *n % 2 == 0));
    println!(
        "sum of squares: {}",
        nums.iter().fold(0, |acc, n| acc + n * n)
    );

    // enumerate + zip + chain
    for (i, v) in nums.iter().enumerate() {
        print!("{i}:{v} ");
    }
    println!();
}
