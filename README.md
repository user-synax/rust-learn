# learn-rust

Runnable Rust notes. One file per concept, each with a `run()` demo.

## Run

```sh
cargo run -- --list      # list all topics
cargo run -- ownership   # run a single topic
cargo run               # run everything in order
```

## Topics (in order)

1. `variables` — let/mut/const/shadowing
2. `data_types` — scalars, tuples, arrays, String vs &str
3. `functions` — signatures, expressions
4. `control_flow` — if/loop/for/match
5. `ownership` — move/borrow/slices (core)
6. `structs` — structs + impl
7. `enums` — enums, Option, Result
8. `error_handling` — panic!, ?, unwrap
9. `collections` — Vec/HashMap/iterators
10. `traits_generics` — traits, generics
11. `lifetimes` — reference lifetimes
12. `closures_iterators` — closures, pipelines
13. `smart_pointers` — Box/Rc/RefCell/Arc
14. `concurrency` — threads, channels, Mutex
15. `file_io` — files, env, args

Start with `docs.md` for the roadmap, then read `src/*.rs` in order.

## Checks

```sh
cargo fmt
cargo clippy -- -D warnings
cargo test
```
