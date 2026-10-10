// cli.rs — STEP 1: Define WHAT your CLI accepts.
//
// WHY THIS FILE?
// In JS you'd hand-parse `process.argv` or use `commander`/`yargs`.
// In Rust the standard is `clap`. With `#[derive(Parser)]` you describe
// your CLI as structs/enums and clap generates --help, validation,
// and parsing for you. No manual argv slicing.
//
// HOW IT WORKS:
// - `Cli` = top-level command (`todo --file x.json <SUBCOMMAND>`)
// - `Commands` = one variant per subcommand (`add`, `list`, ...)
// - clap fills these structs from argv. You then `match` on them in main.rs.
//
// JS MENTAL MODEL:
//   JS: program.command('add <title>').action(...)
//   Rust: enum Commands::Add { title: String } + match in main()

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// STEP 1a: Top-level CLI shape.
///
/// `#[derive(Parser)]` = "implement argument parsing for this struct".
/// `#[command(...)]` = metadata for --help output.
#[derive(Parser, Debug)]
#[command(
    name = "todo",
    version,
    about = "CRUD todo CLI — learn Rust by reading this"
)]
pub struct Cli {
    /// Which subcommand to run (add, list, done, ...).
    ///
    /// WHY `#[command(subcommand)]`?
    /// Tells clap: "this field is not a --flag, it's the subcommand word".
    #[command(subcommand)]
    pub command: Commands,

    /// WHERE to store tasks. Global flag, works with every subcommand.
    ///
    /// WHY `Option<PathBuf>`?
    /// - `Option` = flag may be absent (like `string | undefined` in TS).
    /// - `PathBuf` = owned file path (like `String`, but for paths).
    /// - `global = true` = `--file x.json add ...` and `add ... --file x.json` both work.
    ///
    /// DEFAULT: if user passes nothing, main.rs falls back to "tasks.json".
    #[arg(
        long,
        global = true,
        help = "JSON file to store tasks (default: tasks.json)"
    )]
    pub file: Option<PathBuf>,
}

/// STEP 1b: One variant per operation (this IS your CRUD surface).
///
/// WHY an enum here? Each subcommand has DIFFERENT args.
/// Rust enums hold data (unlike TS string enums), so each variant
/// carries exactly the args it needs. The compiler forces you to
/// handle every variant in `match` — you can't forget one.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// CREATE — add a new task.
    /// Usage: `todo add "buy milk"`
    Add {
        /// Title of the task. Everything after `add` becomes this.
        /// WHY `String` (owned)? clap creates it, gives ownership to us.
        title: String,
    },

    /// READ — list tasks.
    /// Usage: `todo list`, `todo list --done`, `todo list --pending`
    List {
        /// Show only completed tasks.
        /// WHY bool + default false? `--done` present => true, absent => false.
        #[arg(long)]
        done: bool,

        /// Show only pending tasks.
        #[arg(long)]
        pending: bool,
    },

    /// UPDATE — mark a task done.
    /// Usage: `todo done 3`
    Done {
        /// ID printed by `list`. WHY u32? IDs are never negative.
        id: u32,
    },

    /// UPDATE — mark a task back to pending.
    /// Usage: `todo undone 3`
    Undone { id: u32 },

    /// UPDATE — change a task's title.
    /// Usage: `todo edit 3 "buy oat milk"`
    Edit { id: u32, title: String },

    /// DELETE — remove one task.
    /// Usage: `todo remove 3`
    Remove { id: u32 },

    /// DELETE — remove ALL completed tasks.
    /// Usage: `todo clear-done`
    ClearDone,
}
