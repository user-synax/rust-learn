// learn-rust - runnable Rust notes.
// Usage:
//   cargo run                 -> run ALL topics in order
//   cargo run -- variables     -> run one topic
//   cargo run -- --list        -> list topics

mod closures_iterators;
mod collections;
mod concurrency;
mod control_flow;
mod data_types;
mod enums;
mod error_handling;
mod file_io;
mod functions;
mod lifetimes;
mod ownership;
mod smart_pointers;
mod structs;
mod traits_generics;
mod variables;

fn main() {
    let topics: &[(&str, fn())] = &[
        ("variables", variables::run),
        ("data_types", data_types::run),
        ("functions", functions::run),
        ("control_flow", control_flow::run),
        ("ownership", ownership::run),
        ("structs", structs::run),
        ("enums", enums::run),
        ("error_handling", error_handling::run),
        ("collections", collections::run),
        ("traits_generics", traits_generics::run),
        ("lifetimes", lifetimes::run),
        ("closures_iterators", closures_iterators::run),
        ("smart_pointers", smart_pointers::run),
        ("concurrency", concurrency::run),
        ("file_io", file_io::run),
    ];

    let arg = std::env::args().nth(1);

    match arg.as_deref() {
        None => {
            for (name, run) in topics {
                println!("\n===== {name} =====");
                run();
            }
            println!("\nDone. Run one topic: cargo run -- <name>");
        }
        Some("--list") => {
            for (name, _) in topics {
                println!("{name}");
            }
        }
        Some("--help") | Some("-h") => {
            println!("cargo run [-- <topic|--list>]");
        }
        Some(name) => match topics.iter().find(|(n, _)| *n == name) {
            Some((_, run)) => run(),
            None => {
                eprintln!("Unknown topic '{name}'. Available:");
                for (n, _) in topics {
                    eprintln!("  {n}");
                }
                std::process::exit(1);
            }
        },
    }
}
