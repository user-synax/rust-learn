// models.rs — STEP 1: DATA SHAPES. What JSON goes in and out?
//
// A backend is mostly: receive JSON → turn into struct → do logic → turn struct back into JSON.
// `serde` does the JSON <-> struct conversion. `axum::Json` uses it automatically.
//
// JS equivalent:
//   // No structs in JS, just objects:
//   // { id: 1, title: "buy milk", done: false }
//   // Here we DECLARE the shape up front so the compiler checks it.

use serde::{Deserialize, Serialize};

/// STEP 1a: The thing we store AND return.
///
/// `#[derive(...)]` auto-writes code for us:
/// - `Debug`    = lets us `println!("{:?}", task)` while learning
/// - `Clone`    = lets us copy a Task out of the store (see store.rs WHY)
/// - `Serialize`   = struct -> JSON (for RESPONSES)
/// - `Deserialize` = JSON -> struct (so we can also read it back)
///
/// WHY `String`, not `&str`?
/// `String` OWNS its text (heap allocated). Tasks live in a HashMap for a long
/// time, after the request is gone — so they must own their data.
/// `&str` only BORROWS text owned by someone else (dies with the request).
/// Rule of thumb: structs that LIVE ON own Strings; function args BORROW &str.
/// Same lesson as `src/structs.rs` and cli-todo's `task.rs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

/// STEP 1b: What the client sends to CREATE a task.
///
/// POST /tasks  body: `{ "title": "buy milk" }`
/// Notice: NO `id`, NO `done` — the server decides those.
/// `Deserialize` only (we never SEND this shape, only RECEIVE it).
///
/// JS equivalent: `type CreateTask = Pick<Task, "title">`
#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
}

/// STEP 1c: What the client sends to UPDATE a task.
///
/// PUT /tasks/1  body: `{ "title": "new", "done": true }`
/// Both fields are `Option` = "may or may not be present".
/// - `None`    = client didn't send it → leave it alone
/// - `Some(x)` = client sent it → overwrite
///
/// JS equivalent: `{ title?: string, done?: boolean }`
/// Rust has no `undefined` — `Option<T>` IS the `T | undefined`.
#[derive(Debug, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub done: Option<bool>,
}
