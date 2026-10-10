// control_flow.rs - if, loops, for, match
//
// Run: cargo run -- control_flow

pub fn run() {
    println!("--- control_flow ---");

    // 1. if is an expression (returns a value)
    let n = 7;
    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{n} is {label}");

    // 2. loop (infinite, break with value)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // break returns value
        }
    };
    println!("loop result = {result}");

    // 3. while
    let mut x = 3;
    while x > 0 {
        print!("{x} ");
        x -= 1;
    }
    println!();

    // 4. for + ranges + iterators
    for i in 1..=3 {
        // 1..=3 inclusive, 1..4 exclusive
        print!("{i} ");
    }
    println!();

    let fruits = ["apple", "mango", "banana"];
    for (idx, f) in fruits.iter().enumerate() {
        println!("{idx}: {f}");
    }

    // 5. match (exhaustive! compiler forces all cases)
    let code = 404;
    let msg = match code {
        200 => "OK",
        404 => "Not Found",
        500..=599 => "Server Error",
        _ => "Other", // catch-all required
    };
    println!("{code} => {msg}");

    // 6. match with Option
    let maybe: Option<i32> = Some(5);
    match maybe {
        Some(v) => println!("got {v}"),
        None => println!("got nothing"),
    }

    // 7. if let (shorter match for one case)
    if let Some(v) = maybe {
        println!("if let got {v}");
    }

    // 8. while let (pop until empty)
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();

    // 9. loop labels (break outer loop)
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                break 'outer;
            }
            print!("({i},{j}) ");
        }
    }
    println!();
}
