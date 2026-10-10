// main.rs — PRACTICE. Glue cli.rs + task.rs.
//
// TODO 1: mod cli; mod task; use Cli/Commands, Parser, PathBuf.
// TODO 2: fn main() -> Result<()> { parse → resolve file → load → match → save → print }
// TODO 3: Handle every Commands variant. `list` does NOT save, everything else does.
//
// CHECKLIST (do in this order):
//   [ ] Add works: cargo run -- --file /tmp/p.json add "hi"
//   [ ] List works: cargo run -- --file /tmp/p.json list
//   [ ] Done/undone work
//   [ ] Edit/remove/clear-done work
//   [ ] cargo test passes

mod cli;
mod task;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let file: PathBuf = cli.file.unwrap_or_else(|| PathBuf::from("tasks.json"));

    match cli.command {
        Commands::Add { title } => {
            // TODO: load → add_task → save → println!
            let _ = (file, title);
            todo!("Add")
        }
        Commands::List { done, pending } => {
            let _ = (done, pending);
            todo!("List")
        }
        Commands::Done { id } => {
            let _ = id;
            todo!("Done")
        }
        Commands::Undone { id } => {
            let _ = id;
            todo!("Undone")
        }
        Commands::Edit { id, title } => {
            let _ = (id, title);
            todo!("Edit")
        }
        Commands::Remove { id } => {
            let _ = id;
            todo!("Remove")
        }
        Commands::ClearDone => {
            todo!("ClearDone")
        }
    }
}
