# cli-todo — Full CRUD CLI (documented solution)

Learn Rust by READING this code. Every file explains its STEP + WHY + JS equivalent.

## Files = 3 steps

1. `src/cli.rs` — STEP 1: CLI shape with `clap` (what commands exist)
2. `src/task.rs` — STEP 2: `Task` struct + load/save + CRUD on `Vec<Task>`
3. `src/main.rs` — STEP 3: glue (parse → load → match → save → print)

Flow of every run: `JSON file → Vec<Task> → mutate → JSON file`.

## Run

```sh
cargo run -- add "buy milk"
cargo run -- list
cargo run -- list --done
cargo run -- done 1
cargo run -- undone 1
cargo run -- edit 1 "buy oat milk"
cargo run -- remove 1
cargo run -- clear-done
cargo run -- --help

# isolated test file (doesn't touch your real tasks.json):
cargo run -- --file /tmp/test-tasks.json add "hello"
cargo test
```

## What to notice (JS → Rust)

- `Option<PathBuf>` = `string | undefined` for `--file`
- `Result<Vec<Task>>` + `?` = typed `try/catch` that must be handled
- `match command` = exhaustive `switch` (compiler forces all cases)
- `&mut Vec<Task>` = mutating the caller's array in place
- `String` inside structs (owned), `&str`/`&Path` in params (borrowed)
