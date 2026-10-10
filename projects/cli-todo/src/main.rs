// main.rs — STEP 3: GLUE. Parse CLI → load → mutate → save → print.
//
// WHY THIS FILE IS THIN:
// All parsing lives in cli.rs, all data logic in task.rs.
// main() only decides WHICH task.rs function to call.
// Same as JS: index.js routes, logic lives in services.
//
// PROGRAM FLOW (every run):
//   1. Parse argv into `Cli` (clap)
//   2. Resolve file path (--file or tasks.json)
//   3. load() JSON → Vec<Task>
//   4. match on command → mutate Vec
//   5. save() Vec → JSON (for read-only `list`, skip save)
//   6. println! result. Errors bubble up via `?` and print nicely.

mod cli;
mod task;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::path::PathBuf;

/// STEP 3a: Entry point returns `Result<()>`, not `()`.
///
/// WHY? `fn main() -> Result<()>` + `?` lets any error (IO, JSON)
/// return early and Rust prints `Error: <context>: <cause>`.
/// JS equivalent: `async main() { try { ... } catch(e){ console.error(e) } }`
/// — except here it's enforced, not optional.
fn main() -> Result<()> {
    // STEP 1: Parse argv. `Cli::parse()` reads std::env::args, validates,
    // prints --help / errors and exits on its own if argv is bad.
    let cli = Cli::parse();

    // STEP 2: Which file? `--file custom.json` or default `tasks.json`.
    // WHY `unwrap_or`? `Option<PathBuf>` → real `PathBuf`.
    // Like JS: `const file = args.file ?? "tasks.json"`.
    let file: PathBuf = cli.file.unwrap_or_else(|| PathBuf::from("tasks.json"));

    // STEP 3: Dispatch. `match` MUST cover every Commands variant —
    // add a variant in cli.rs and this stops compiling until you handle it.
    // (TS switch would silently fall through. Rust won't let you.)
    match cli.command {
        // --- CREATE ---
        Commands::Add { title } => {
            let mut tasks = task::load(&file)?;
            // `title` is owned String from clap → moved into add_task, no clone.
            let created = task::add_task(&mut tasks, title);
            task::save(&file, &tasks)?;
            println!("#{} added: {}", created.id, created.title);
        }

        // --- READ ---
        Commands::List { done, pending } => {
            let tasks = task::load(&file)?;
            // Filter view only — no save needed (read-only command).
            // `.iter().filter(...)` = JS `.filter(...)`, lazy until `for`.
            for t in tasks.iter().filter(|t| {
                if done {
                    t.done
                } else if pending {
                    !t.done
                } else {
                    true
                }
            }) {
                // `[x]` / `[ ]` like GitHub task lists. Human output, not JSON.
                let mark = if t.done { "x" } else { " " };
                println!("#{} [{}] {}", t.id, mark, t.title);
            }
            if tasks.is_empty() {
                println!("No tasks yet. Add one: todo add \"my first task\"");
            }
        }

        // --- UPDATE (done / undone / edit) ---
        Commands::Done { id } => {
            let mut tasks = task::load(&file)?;
            let updated = task::set_done(&mut tasks, id, true)?;
            task::save(&file, &tasks)?;
            println!("#{} done: {}", updated.id, updated.title);
        }
        Commands::Undone { id } => {
            let mut tasks = task::load(&file)?;
            let updated = task::set_done(&mut tasks, id, false)?;
            task::save(&file, &tasks)?;
            println!("#{} pending: {}", updated.id, updated.title);
        }
        Commands::Edit { id, title } => {
            let mut tasks = task::load(&file)?;
            let updated = task::edit_title(&mut tasks, id, title)?;
            task::save(&file, &tasks)?;
            println!("#{} updated: {}", updated.id, updated.title);
        }

        // --- DELETE ---
        Commands::Remove { id } => {
            let mut tasks = task::load(&file)?;
            let removed = task::remove_task(&mut tasks, id)?;
            task::save(&file, &tasks)?;
            println!("#{} removed: {}", removed.id, removed.title);
        }
        Commands::ClearDone => {
            let mut tasks = task::load(&file)?;
            let n = task::clear_done(&mut tasks);
            task::save(&file, &tasks)?;
            println!("Cleared {n} completed task(s).");
        }
    }

    // `Ok(())` = success, no value (like `return;` in JS, but explicit).
    Ok(())
}
