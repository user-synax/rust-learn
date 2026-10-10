# Rust: Zero to Real-World Projects

This repo is runnable notes. Each `src/*.rs` is one concept with a `run()` demo.
Start here, then read files in the order `main.rs` lists them.

```sh
cargo run -- --list     # list topics
cargo run -- ownership  # run one topic
cargo run              # run everything
```

---

## 1. Mental model (read before code)

Rust has no garbage collector and no manual `free`. Instead:

- **Ownership**: every value has one owner. When owner goes out of scope, value is dropped. `src/ownership.rs`.
- **Borrowing**: pass `&T` (read) or `&mut T` (write). Either many `&T` OR one `&mut T`, never both. Enforced at compile time → no data races.
- **No null**: absent value = `Option<T>` (`Some`/`None`). Failure = `Result<T, E>` (`Ok`/`Err`).
- **Expressions**: `if`, `match`, blocks return values. Last line without `;` is the return.
- **Exhaustiveness**: `match` must cover all cases. The compiler is your pair programmer — listen to it.

If something doesn't compile, 90% of the time it's ownership, types, or an unhandled `Option`/`Result`.

## 2. Suggested reading order

| Order | File | Why it matters |
|-------|------|----------------|
| 1 | `variables.rs` | `let`/`mut`/`const`, shadowing |
| 2 | `data_types.rs` | ints, floats, tuples, arrays, `String` vs `&str` |
| 3 | `functions.rs` | signatures, expressions vs statements |
| 4 | `control_flow.rs` | `if`/`loop`/`for`/`match`, `if let` |
| 5 | `ownership.rs` | **most important file** — move/clone/borrow/slices |
| 6 | `structs.rs` | data + `impl` methods |
| 7 | `enums.rs` | enums with data, `Option`, `Result` |
| 8 | `error_handling.rs` | `panic!` vs `Result`, `?`, `unwrap_or` |
| 9 | `collections.rs` | `Vec`, `HashMap`, iterators |
| 10 | `traits_generics.rs` | traits (~interfaces), generics, trait objects |
| 11 | `lifetimes.rs` | when output borrows input |
| 12 | `closures_iterators.rs` | `map`/`filter`/`collect` pipelines |
| 13 | `smart_pointers.rs` | `Box`, `Rc`, `RefCell`, `Arc` |
| 14 | `concurrency.rs` | threads, channels, `Arc<Mutex<T>>` |
| 15 | `file_io.rs` | files, env, args, stdout/stderr |

## 3. Tooling you must know

```sh
cargo new myapp        # new binary project
cargo new --lib mylib  # new library
cargo run              # build + run
cargo run -- <args>    # args after --
cargo check            # fast typecheck (use while coding)
cargo clippy           # linter — fix all warnings
cargo fmt              # formatter — run before commit
cargo test             # run #[test]s
cargo doc --open       # docs for your code + deps
rustup update          # update toolchain
```

Project layout for real apps:

```
src/main.rs        # binary entry (or src/lib.rs for library)
src/routes/        # axum/actix handlers
src/models.rs      # structs + serde
Cargo.toml         # deps + features
tests/             # integration tests
```

`Cargo.toml` basics:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"     # easy errors in apps
thiserror = "1"  # typed errors in libs
```

## 4. Error handling strategy (real projects)

- Apps: return `anyhow::Result<T>`, use `?` everywhere, `.context("...")` for messages.
- Libraries: define typed errors with `thiserror`, never `panic!` on user input.
- Never `unwrap()` on IO/network/parse in production. `expect()` only with a reason why it's impossible.
- Validate at boundaries (CLI args, HTTP body, env), then use plain types inside.

## 5. Ownership cheat sheet

```rust
String::from("x")      // owned, mutable, heap
"literal"              // &'static str, borrowed
&s                     // borrow as &String -> coerces to &str
&s[..]                 // full slice
s.clone()              // explicit deep copy
&mut s                 // exclusive mutable borrow
```

Function signatures to default to:

```rust
fn read(s: &str);          // not String, unless you need ownership
fn push(v: &mut Vec<i32>); // mutate in place
fn make() -> String;       // return owned
```

## 6. What to learn next (after this repo)

1. **Testing**: `#[cfg(test)]`, `cargo test`, `assert_eq!`, integration tests in `tests/`.
2. **Crates**: `serde` (JSON), `tokio` (async), `axum` (API), `clap` (CLI), `sqlx` (DB).
3. **Async**: `async fn`, `.await`, `tokio::main`. Same ownership rules, just non-blocking.
4. **Modules**: `mod foo;` + `foo.rs`, `pub`, `use crate::...`. Split files when `main.rs` > ~300 lines.
5. **Lifetimes in practice**: you rarely write them except for functions returning references. Structs holding `&str` usually become `String` in real apps to avoid lifetime plumbing.

## 7. Common beginner traps

- `expected String, found &str` → call `.to_string()` / `String::from(...)`.
- `cannot move out of borrowed content` → clone, or take `&` instead of owned.
- `cannot borrow as mutable more than once` → restructure: compute first, mutate after; or use channels instead of shared `&mut`.
- Fighting the borrow checker with `Rc<RefCell<>>` everywhere → usually means the design should pass owned data or use indexes/channels instead.
- `unwrap()` in a loop over IO → use `?` + `match`.

## 8. How to verify you got it

You can build real projects when you can, without looking up:

1. Explain move vs borrow to someone else.
2. Parse CLI args → read a file → transform with iterators → write output, with `Result` errors (no unwrap).
3. Define a struct + enum + trait, drive them with a `match`.
4. Spin 5 threads that send results over a channel and sum them.
5. Write 3 unit tests and run `cargo test`, `cargo clippy`, `cargo fmt --check` clean.
