// cli.rs — PRACTICE. Rebuild this from memory, then peek at ../cli-todo/src/cli.rs.
//
// TODO 1: Define `Cli` struct with #[derive(Parser)].
//   Fields: command (subcommand), file: Option<PathBuf> (global --file flag).
//
// TODO 2: Define `Commands` enum with #[derive(Subcommand)].
//   Variants: Add{title}, List{done,pending}, Done{id}, Undone{id},
//             Edit{id,title}, Remove{id}, ClearDone.
//
// HINT: each variant's fields become that subcommand's args automatically.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "todo", version, about = "Practice rebuild")]
pub struct Cli {
    // DONE FOR YOU (needed to compile) — your practice is in Commands + task.rs + main.rs.
    // `#[command(subcommand)]` = "this field is the subcommand word, not a --flag".
    #[command(subcommand)]
    pub command: Commands,
    // `#[arg(long, global = true)]` = "--file x.json works with every subcommand".
    #[arg(long, global = true)]
    pub file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // TODO: fill in all 7 variants. Start with Add + List, get those working,
    // then add the rest one by one.
    Add { title: String },
    List { done: bool, pending: bool },
    Done { id: u32 },
    Undone { id: u32 },
    Edit { id: u32, title: String },
    Remove { id: u32 },
    ClearDone,
}
