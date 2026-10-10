// file_io.rs - Files, env, args, JSON-style parsing without crates
//
// Run: cargo run -- file_io
// Real projects use: serde + serde_json, tokio::fs, clap, anyhow.

use std::env;
use std::fs;
use std::io::{self, Write};

pub fn run() {
    println!("--- file_io ---");

    // --- CLI args ---
    let args: Vec<String> = env::args().collect();
    println!("args = {:?}", args);

    // --- Env vars ---
    // SAFETY: env::set_var is safe here (single-threaded demo).
    unsafe { env::set_var("APP_MODE", "demo") };
    match env::var("APP_MODE") {
        Ok(v) => println!("APP_MODE = {v}"),
        Err(_) => println!("APP_MODE not set"),
    }

    // --- Write + read file ---
    let path = "demo_output.txt";
    let content = "hello from rust\nline 2\n";
    fs::write(path, content).expect("write failed");
    println!("wrote {path}");

    let read_back = fs::read_to_string(path).expect("read failed");
    println!("read back:\n{read_back}");

    // --- Read lines ---
    let lines: Vec<&str> = read_back.lines().collect();
    println!("{} lines: {:?}", lines.len(), lines);

    // --- stdout / stderr ---
    print!("stdout print (no newline)... ");
    io::stdout().flush().unwrap();
    println!("flushed!");
    eprintln!("(this goes to stderr)");

    // --- Cleanup ---
    fs::remove_file(path).ok();
    println!("cleaned up {path}");
}
