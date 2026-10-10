# cli-todo-practice — Rebuild it yourself

Solution is in `../cli-todo/`. Don't copy-paste — rebuild file by file.

## Order

1. `src/cli.rs` — already stubbed, check `--help` works: `cargo run -- --help`
2. `src/task.rs` — implement `new → next_id → add_task → set_done → edit/remove/clear → load/save`
   - After each fn: `cargo test`
3. `src/main.rs` — wire match arms one by one against `/tmp/p.json`:
```sh
cargo run -- --file /tmp/p.json add "hi"
cargo run -- --file /tmp/p.json list
cargo run -- --file /tmp/p.json done 1
cargo run -- --file /tmp/p.json edit 1 "hello"
cargo run -- --file /tmp/p.json remove 1
```

## Done when

- All 3 `cargo test` tests pass
- All 7 commands work with `--file /tmp/p.json`
- You can explain: why `&mut Vec`, why `?`, why `match` forces all cases
