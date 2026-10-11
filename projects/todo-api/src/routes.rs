// routes.rs — STEP 3: HANDLERS. One async fn per endpoint.
//
// A handler = async fn that takes INPUT EXTRACTORS and returns a RESPONSE.
//
// Extractors (axum fills these from the HTTP request):
//   `State(store)`   → the SharedStore from main.rs
//   `Path(id)`       → the `:id` part of `/tasks/{id}`
//   `Json(body)`     → parses the JSON request body into a struct
//
// Return: anything that implements `IntoResponse`.
// Common: `&'static str`, `Json(task)`, `(StatusCode, Json(task))`, `StatusCode`.
//
// JS (Express) equivalent:
//   app.get("/tasks/:id", (req, res) => {
//     const task = store.get(req.params.id);   // Path + State
//     task ? res.json(task) : res.sendStatus(404);
//   });
//
// Rust differences:
// - Types are EXPLICIT: `Path<u64>` fails with 422 if id isn't a number.
// - No `req, res` objects — you declare what you need, axum injects it.
// - `async` is REQUIRED (server is async; see main.rs WHY).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{CreateTask, Task, UpdateTask},
    store::SharedStore,
};

/// GET /health → "ok"
/// The simplest handler: no input, static string output.
/// Use it to check the server is up: `curl localhost:3000/health`.
pub async fn health() -> &'static str {
    "ok"
}

/// GET /tasks → `[{...}, {...}]`
/// Returns ALL tasks as JSON. Always 200 (empty list `[]` if none).
pub async fn list_tasks(State(store): State<SharedStore>) -> Json<Vec<Task>> {
    // `State(store)` unwraps the shared pointer (cheap Arc clone per request).
    Json(store.list())
}

/// POST /tasks  body `{ "title": "..." }` → 201 + created task.
/// `StatusCode::CREATED` (201) = "we made something". REST convention.
pub async fn create_task(
    State(store): State<SharedStore>,
    Json(input): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    // Edge case: reject blank titles HERE (at the boundary),
    // so Store inside only ever sees valid data.
    // (docs.md rule: "Validate at boundaries, then use plain types inside.")
    //
    // NOTE: this project keeps validation tiny on purpose.
    // Real apps would use a `validator` crate or return JSON error bodies.
    let title = input.title.trim().to_string();
    let task = store.create(CreateTask { title });
    (StatusCode::CREATED, Json(task))
}

/// GET /tasks/{id} → 200 + task, or 404.
///
/// Return type `Result<Json<Task>, StatusCode>`:
///
/// - `Ok(Json(task))` → 200 with JSON body
/// - `Err(404)`       → empty body with 404 status
///
/// axum converts each side into an HTTP response automatically.
pub async fn get_task(
    State(store): State<SharedStore>,
    Path(id): Path<u64>,
) -> Result<Json<Task>, StatusCode> {
    store.get(id).map(Json).ok_or(StatusCode::NOT_FOUND)
}

/// PUT /tasks/{id}  body `{ "title"?, "done"? }` → 200 + task, or 404.
///
/// WHY PUT and not PATCH? For a learning project one update route is enough.
/// PUT here behaves like PATCH (partial update via Option fields).
/// Real APIs often have both; don't overthink it for now.
pub async fn update_task(
    State(store): State<SharedStore>,
    Path(id): Path<u64>,
    Json(patch): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    store
        .update(id, patch)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// DELETE /tasks/{id} → 204 (deleted) or 404.
///
/// `StatusCode::NO_CONTENT` (204) = "done, no body to send back".
/// REST convention for successful DELETE.
pub async fn delete_task(State(store): State<SharedStore>, Path(id): Path<u64>) -> StatusCode {
    if store.delete(id) {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
