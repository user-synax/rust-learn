// task.rs — STEP 2: Your DATA + CRUD logic (no CLI code here).
//
// WHY THIS FILE?
// Separation of concerns: `cli.rs` = "what did the user type?",
// `task.rs` = "what does a task look like + how do we store it?",
// `main.rs` = "glue them together".
// Same as JS: routes/ + models/ + db.js split.
//
// HOW IT WORKS (big picture):
//   JSON file on disk  <---load--->  Vec<Task> in memory  <---mutate--->  save back
// Every command = load → change Vec → save. Simple, no database needed.
//
// JS MENTAL MODEL:
//   JS: { id: 1, title: "x", done: false }  (plain object, JSON = same shape)
//   Rust: struct Task { ... } + #[derive(Serialize, Deserialize)] makes it JSON-able.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// STEP 2a: The Task struct — your "model".
// ---------------------------------------------------------------------------

/// One todo item.
///
/// WHY `#[derive(...)]`?
/// - `Serialize/Deserialize` = convert to/from JSON (like JSON.parse/stringify, but typed).
/// - `Debug/Clone` = printable + copyable for learning/debugging.
///
/// WHY THESE TYPES?
/// - `u32` for id: tasks are never negative, 0 = invalid sentinel.
/// - `String` (owned) for title: Task OWNS its title. If we used `&str`
///   (borrowed), every Task would borrow from somewhere else and we'd need
///   lifetimes everywhere. Real apps use `String` inside structs. (See lifetimes.rs.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    /// Constructor. WHY a `new` fn? Rust has no `new` keyword / classes.
    /// Convention: `Type::new(...)` = like `new Task(...)` in JS.
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            done: false,
        }
    }
}

// ---------------------------------------------------------------------------
// STEP 2b: LOAD — file → Vec<Task>.
// ---------------------------------------------------------------------------

/// Read tasks from `path`. Missing file = empty list (not an error).
///
/// WHY `Result<Vec<Task>>`?
/// In JS you'd `throw` on read/parse failure. In Rust you RETURN the
/// failure: `Ok(tasks)` or `Err(e)`. Caller decides with `match` or `?`.
///
/// WHY `&Path`? We only need to BORROW the path to read it, not own it.
/// (Ownership 101: `&` = look, don't take.)
pub fn load(path: &Path) -> Result<Vec<Task>> {
    // STEP: file doesn't exist yet (first run) → start empty.
    // `path.exists()` avoids an error on fresh installs.
    if !path.exists() {
        return Ok(Vec::new());
    }

    // STEP: read whole file to String.
    // `.with_context(...)` = add "while reading tasks.json" to any error,
    // like wrapping a JS error with extra message. Needs `anyhow::Context`.
    let data =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;

    // STEP: empty file (0 bytes) → empty list, not a JSON error.
    if data.trim().is_empty() {
        return Ok(Vec::new());
    }

    // STEP: JSON → Vec<Task>. `serde_json::from_str` = JSON.parse, but typed.
    // `?` = "if parse fails, return that error NOW" (like throw, but explicit).
    let tasks: Vec<Task> = serde_json::from_str(&data)
        .with_context(|| format!("{} is not valid JSON", path.display()))?;
    Ok(tasks)
}

// ---------------------------------------------------------------------------
// STEP 2c: SAVE — Vec<Task> → file.
// ---------------------------------------------------------------------------

/// Write tasks to `path` as pretty JSON.
///
/// WHY `&[Task]` (slice) and not `Vec<Task>`?
/// `&[Task]` = "borrowed view of any list". Accepts `&Vec<Task>`,
/// `&array`, `&slice` — most flexible, takes no ownership.
/// JS equivalent: passing an array you promise not to reassign.
pub fn save(path: &Path, tasks: &[Task]) -> Result<()> {
    // `to_string_pretty` = JSON.stringify(tasks, null, 2).
    let data = serde_json::to_string_pretty(tasks).context("failed to encode tasks as JSON")?;
    fs::write(path, data).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

// ---------------------------------------------------------------------------
// STEP 2d: CRUD helpers — pure Vec operations (easy to unit-test).
// ---------------------------------------------------------------------------

/// Next free ID = max existing + 1. Starts at 1.
///
/// WHY not `tasks.len() + 1`? After deleting task 2 of [1,2,3],
/// len+1 would reuse ID 3 → collisions. max+1 never collides.
pub fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

/// CREATE — push a new task, return it (cloned so caller can print it).
pub fn add_task(tasks: &mut Vec<Task>, title: String) -> Task {
    // WHY `&mut Vec`? We mutate the caller's list in place (push).
    // JS: `tasks.push(...)` mutates the same array reference.
    let task = Task::new(next_id(tasks), title);
    tasks.push(task.clone()); // clone = explicit copy (Strings live on heap)
    task
}

/// UPDATE — find by id, mutate. Returns the updated task or an error.
///
/// WHY `iter_mut().find(...)`?
/// - `iter_mut()` = mutable iterator (lets us change the found task).
/// - `find(|t| ...)` = like JS `.find(t => t.id === id)`.
/// - Returns `Option<&mut Task>` → `None` if no such ID.
/// - `.ok_or_else(...)` turns None into Err (like `throw new Error`).
///
/// NOTE: `&mut Vec<Task>` is used instead of `&mut [Task]` on purpose —
/// clearer for learning (`Vec` = the growable list you already know).
#[allow(clippy::ptr_arg)]
pub fn set_done(tasks: &mut Vec<Task>, id: u32, done: bool) -> Result<Task> {
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| anyhow::anyhow!("no task with id {id}"))?;
    task.done = done;
    Ok(task.clone())
}

/// UPDATE — rename a task.
#[allow(clippy::ptr_arg)]
pub fn edit_title(tasks: &mut Vec<Task>, id: u32, title: String) -> Result<Task> {
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| anyhow::anyhow!("no task with id {id}"))?;
    task.title = title; // `title: String` moved in — no clone needed
    Ok(task.clone())
}

/// DELETE — remove one task. Returns it so main.rs can print what was removed.
///
/// WHY `position` + `remove` (two steps)?
/// `Vec::remove(i)` needs the INDEX, but user gives us the ID.
/// So: 1) find index of matching ID, 2) remove by index.
/// JS: `const i = arr.findIndex(...); arr.splice(i, 1)`.
pub fn remove_task(tasks: &mut Vec<Task>, id: u32) -> Result<Task> {
    let pos = tasks
        .iter()
        .position(|t| t.id == id)
        .ok_or_else(|| anyhow::anyhow!("no task with id {id}"))?;
    Ok(tasks.remove(pos))
}

/// DELETE — drop all done tasks. Returns HOW MANY were removed.
///
/// WHY `retain`? Keeps only items where closure returns true.
/// JS: `tasks = tasks.filter(t => !t.done)` — but `retain` edits in place,
/// no new allocation, no ownership move.
pub fn clear_done(tasks: &mut Vec<Task>) -> usize {
    let before = tasks.len();
    tasks.retain(|t| !t.done);
    before - tasks.len()
}

// ---------------------------------------------------------------------------
// STEP 2e: Unit tests — prove CRUD works without touching files/CLI.
// Run with: cargo test
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assigns_incrementing_ids() {
        let mut tasks = Vec::new();
        let a = add_task(&mut tasks, "one".to_string());
        let b = add_task(&mut tasks, "two".to_string());
        assert_eq!(a.id, 1);
        assert_eq!(b.id, 2);
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn done_and_undone_flip_flag() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, "x".to_string());
        set_done(&mut tasks, 1, true).unwrap();
        assert!(tasks[0].done);
        set_done(&mut tasks, 1, false).unwrap();
        assert!(!tasks[0].done);
    }

    #[test]
    fn unknown_id_is_an_error() {
        let mut tasks = Vec::new();
        assert!(set_done(&mut tasks, 99, true).is_err());
        assert!(remove_task(&mut tasks, 99).is_err());
    }
}
