// task.rs — PRACTICE. Implement each function, run `cargo test` after each one.
//
// ORDER:
//   1. Task struct + new()
//   2. next_id() + add_task() → cargo test add_assigns_incrementing_ids
//   3. set_done() → cargo test done_and_undone_flip_flag
//   4. edit_title() + remove_task() + clear_done()
//   5. load() + save() → cargo run -- --file /tmp/p.json add "hi"
//
// RULES: no unwrap() on file/parse logic — return Result, use ? and with_context.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        // TODO: construct Self { id, title, done: false }
        todo!("Task::new")
    }
}

// TODO: load(path: &Path) -> Result<Vec<Task>>
//   - if !path.exists() → Ok(vec![])
//   - read_to_string + with_context
//   - empty file → Ok(vec![])
//   - serde_json::from_str + with_context
pub fn load(path: &Path) -> Result<Vec<Task>> {
    todo!("load")
}

// TODO: save(path: &Path, tasks: &[Task]) -> Result<()>
//   - to_string_pretty + fs::write, both with context
pub fn save(path: &Path, tasks: &[Task]) -> Result<()> {
    todo!("save")
}

// TODO: next_id(tasks: &[Task]) -> u32 — max id + 1, or 1 if empty
pub fn next_id(tasks: &[Task]) -> u32 {
    todo!("next_id")
}

// TODO: add_task(tasks: &mut Vec<Task>, title: String) -> Task
pub fn add_task(tasks: &mut Vec<Task>, title: String) -> Task {
    todo!("add_task")
}

// TODO: set_done(tasks: &mut Vec<Task>, id: u32, done: bool) -> Result<Task>
//   HINT: iter_mut().find(|t| t.id == id).ok_or_else(|| anyhow::anyhow!(...))
pub fn set_done(tasks: &mut Vec<Task>, id: u32, done: bool) -> Result<Task> {
    todo!("set_done")
}

// TODO: edit_title(...) — same pattern, assign task.title = title
pub fn edit_title(tasks: &mut Vec<Task>, id: u32, title: String) -> Result<Task> {
    todo!("edit_title")
}

// TODO: remove_task(...) — position() then tasks.remove(pos)
pub fn remove_task(tasks: &mut Vec<Task>, id: u32) -> Result<Task> {
    todo!("remove_task")
}

// TODO: clear_done(...) -> usize — retain(|t| !t.done), return removed count
pub fn clear_done(tasks: &mut Vec<Task>) -> usize {
    todo!("clear_done")
}

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
