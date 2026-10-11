// store.rs — STEP 2: SHARED MEMORY. Where tasks live between requests?
//
// The server runs MANY requests at once (async tasks on threads).
// They all share ONE HashMap. Two problems to solve:
//
//   PROBLEM 1: Many threads, one HashMap → data race?
//   FIX: `Mutex` — only one thread touches the map at a time.
//   Like JS: JS is single-threaded so one request runs at a time.
//   Rust is multi-threaded, so the lock is EXPLICIT.
//
//   PROBLEM 2: Many owners of the same Store (one per request handler)?
//   FIX: `Arc` (Atomic Reference Counted) — shared ownership.
//   Like JS: every closure just closes over the same object.
//   Rust: ownership is strict, so `Arc::clone(&store)` gives each
//   handler its own cheap pointer to the SAME Store.
//
// So: `Arc<Store>` where `Store` contains `Mutex<HashMap<...>>`.
// This pattern — `Arc<Mutex<T>>` — is THE standard shared-state pattern.
// You already saw it in `src/concurrency.rs` and `src/smart_pointers.rs`.

use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};

use crate::models::{CreateTask, Task, UpdateTask};

/// STEP 2a: The database (in memory).
///
/// - `tasks`: the actual map, behind a `Mutex` (locked on each access).
/// - `next_id`: counter for new ids. `AtomicU64` = lock-free integer
///   that can be safely bumped from many threads at once.
///
/// WHY `Mutex` from `std`, not `tokio::sync::Mutex`?
/// Our lock is held for MICROSECONDS (one HashMap insert).
/// `std::sync::Mutex` is faster for that. `tokio::Mutex` is only needed
/// if you `.await` WHILE holding the lock — we never do.
#[derive(Debug, Default)]
pub struct Store {
    next_id: AtomicU64,
    tasks: Mutex<HashMap<u64, Task>>,
}

/// STEP 2b: The type handlers actually receive.
/// `Arc<Store>` = "shared pointer to Store".
/// Cloning an Arc is CHEAP (bumps a counter, doesn't copy the map).
pub type SharedStore = std::sync::Arc<Store>;

impl Store {
    /// Create an empty store. Called ONCE in main.rs, then shared.
    pub fn new() -> Self {
        Self {
            // Start ids at 1 so `#0` never exists (nicer for humans/curl).
            next_id: AtomicU64::new(1),
            tasks: Mutex::new(HashMap::new()),
        }
    }

    /// READ ALL: clone every Task into a Vec.
    ///
    /// WHY clone? The Tasks live INSIDE the locked HashMap.
    /// We can't hand out references — the lock unlocks when this
    /// function returns, so references would dangle.
    /// Cloning gives the caller OWNED copies that live on their own.
    /// (Same reason cli-todo returns owned Strings, not &str.)
    pub fn list(&self) -> Vec<Task> {
        // `.lock().unwrap()` = "wait for my turn, then give me the map".
        // `unwrap` here: a poisoned mutex means another thread panicked
        // while holding it — crashing is correct for a learning project.
        // Real prod code would map this to a 500 error instead.
        let map = self.tasks.lock().unwrap();
        let mut out: Vec<Task> = map.values().cloned().collect();
        // Sort so GET /tasks is deterministic (HashMap order is random).
        out.sort_by_key(|t| t.id);
        out
    }

    /// CREATE: allocate id → build Task → insert → return a CLONE.
    pub fn create(&self, input: CreateTask) -> Task {
        // `fetch_add` atomically returns the OLD value and bumps by 1.
        // Two threads calling at once get DIFFERENT ids. No lock needed.
        // `Ordering::Relaxed`: fine here — ids just need uniqueness, not sync with other data.
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let task = Task {
            id,
            title: input.title, // `input` is owned → we MOVE its String in, no clone.
            done: false,        // new tasks always start pending. Server decides, not client.
        };
        self.tasks.lock().unwrap().insert(id, task.clone());
        task // return owned copy to the handler, which serializes it to JSON.
    }

    /// READ ONE: `Option<Task>` = "maybe found".
    /// `Some(task)` → 200, `None` → 404 (routes.rs decides the status code).
    pub fn get(&self, id: u64) -> Option<Task> {
        self.tasks.lock().unwrap().get(&id).cloned()
    }

    /// UPDATE: patch whichever fields are `Some`, return updated copy.
    /// Returns `None` if id doesn't exist.
    pub fn update(&self, id: u64, patch: UpdateTask) -> Option<Task> {
        let mut map = self.tasks.lock().unwrap();
        // `get_mut` = mutable borrow of the Task INSIDE the map.
        // We mutate it in place, then clone it out for the response.
        let task = map.get_mut(&id)?;
        // `?` on Option: if `get_mut` gave None, return None early. Else unwrap.
        if let Some(title) = patch.title {
            task.title = title;
        }
        if let Some(done) = patch.done {
            task.done = done;
        }
        Some(task.clone())
    }

    /// DELETE: returns `true` if something was removed.
    pub fn delete(&self, id: u64) -> bool {
        self.tasks.lock().unwrap().remove(&id).is_some()
    }
}

// STEP 2c: Unit tests. `cargo test` runs these.
// Testing the Store WITHOUT http proves logic and web layers are separate.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_then_get() {
        let s = Store::new();
        let t = s.create(CreateTask {
            title: "buy milk".into(),
        });
        assert_eq!(t.id, 1);
        assert!(!t.done);
        assert_eq!(s.get(1).unwrap().title, "buy milk");
    }

    #[test]
    fn update_patches_only_sent_fields() {
        let s = Store::new();
        s.create(CreateTask { title: "a".into() });
        let u = s
            .update(
                1,
                UpdateTask {
                    title: None,
                    done: Some(true),
                },
            )
            .unwrap();
        assert_eq!(u.title, "a"); // untouched
        assert!(u.done); // patched
    }

    #[test]
    fn delete_missing_returns_false() {
        let s = Store::new();
        assert!(!s.delete(999));
    }
}
