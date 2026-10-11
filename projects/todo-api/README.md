# todo-api — Your First Rust Backend (documented solution)

Learn Rust backends by READING this code. Every file is one STEP with WHY + JS equivalent.

> If you understood `projects/cli-todo`, you already know 60% of this.
> CLI was: `argv → struct → logic → print`.
> Backend is: `HTTP → struct → logic → JSON`. Same shape, different boundary.

## Run

```sh
cd projects/todo-api
cargo run
# new terminal:
curl localhost:3000/health
```

Full CRUD test (copy-paste):

```sh
# CREATE
curl -X POST localhost:3000/tasks -H 'Content-Type: application/json' -d '{"title":"buy milk"}'
curl -X POST localhost:3000/tasks -H 'Content-Type: application/json' -d '{"title":"read docs.md"}'

# READ ALL / READ ONE
curl localhost:3000/tasks
curl localhost:3000/tasks/1

# UPDATE (partial — send only what changes)
curl -X PUT localhost:3000/tasks/1 -H 'Content-Type: application/json' -d '{"done":true}'
curl -X PUT localhost:3000/tasks/1 -H 'Content-Type: application/json' -d '{"title":"buy oat milk"}'

# DELETE (204 = gone, no body)
curl -i -X DELETE localhost:3000/tasks/2
curl localhost:3000/tasks/1      # 404 after delete
curl localhost:3000/tasks/99     # 404 — never existed

# BAD INPUT (axum rejects before your code runs)
curl localhost:3000/tasks/abc    # 422 — {id} must be a number
```

Checks (same as root repo):

```sh
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## API table

| Method | Path         | Body                              | Success      | Miss   |
|--------|--------------|-----------------------------------|--------------|--------|
| GET    | `/health`    | —                                 | 200 `"ok"`   | —      |
| GET    | `/tasks`     | —                                 | 200 `[...]`  | —      |
| POST   | `/tasks`     | `{"title":"..."}`                 | 201 `{...}`  | —      |
| GET    | `/tasks/{id}`| —                                 | 200 `{...}`  | 404    |
| PUT    | `/tasks/{id}`| `{"title"?,"done"?}` (partial)   | 200 `{...}`  | 404    |
| DELETE | `/tasks/{id}`| —                                 | 204 (empty)  | 404    |

Task shape: `{ "id": 1, "title": "buy milk", "done": false }`

Status codes used: `200 OK`, `201 Created`, `204 No Content`, `404 Not Found`, `422 Bad Input`.

## Files = 4 steps

```
src/models.rs  — STEP 1: JSON shapes (Task, CreateTask, UpdateTask) + serde
src/store.rs   — STEP 2: shared memory (Arc + Mutex + HashMap) + unit tests
src/routes.rs  — STEP 3: one async handler per endpoint
src/main.rs    — STEP 4: glue (state → router → serve on :3000)
```

Read them **in order 1 → 4**. Each file header explains its STEP + WHY + JS equivalent.

Request flow (every request):

```
curl → axum Router → extractor (State/Path/Json) → handler in routes.rs
     → Store method in store.rs (lock Mutex, touch HashMap)
     → Json response + status code
```

Compare with cli-todo flow: `JSON file → Vec<Task> → mutate → JSON file`.
Here: `HTTP body → struct → HashMap → JSON response`. Storage differs, pattern is identical.

## What to notice (JS → Rust)

| JS (Express)              | Rust (axum)                              | File              |
|---------------------------|------------------------------------------|-------------------|
| `req.params.id`           | `Path(id): Path<u64>`                    | `routes.rs:70`    |
| `req.body` + `app.use(json())` | `Json(x): Json<CreateTask>`         | `routes.rs:43`    |
| shared `const store = {}` | `Arc<Store>` cloned per handler          | `store.rs:40`     |
| single-threaded, no locks | `Mutex<HashMap>` — lock is explicit      | `store.rs:30`     |
| `obj \| undefined`        | `Option<Task>` → 200 or 404              | `routes.rs:66`    |
| `res.status(201).json(t)` | `(StatusCode::CREATED, Json(t))`         | `routes.rs:46`    |
| `app.listen(3000)`        | `axum::serve(listener, app).await`       | `main.rs:46`      |
| event loop (Node)         | `#[tokio::main]` runtime                 | `main.rs:27`      |

Rust concepts this drills (mapped to root `src/*.rs`):

- `structs` + `enums` — `Task` struct, `Option`/`Result` for 404s
- `ownership` — `Arc::clone` is cheap pointer copy, `task.clone()` is real data copy; handlers MOVE owned Strings into the store
- `smart_pointers` + `concurrency` — `Arc<Mutex<HashMap>>` is the standard shared-state pattern
- `error_handling` — `Result<Json, StatusCode>` instead of throw; `?` on `Option` in `store.rs`
- `traits_generics` — handlers return `impl IntoResponse` (axum converts many types to HTTP)
- `lifetimes` — avoided on purpose: store OWNS `String`s so no lifetimes needed (docs.md §6.5 rule)

## Exercises (do these next)

1. **POST validation**: reject empty titles with `400` + JSON error body instead of creating. Hint: handler return type becomes `Result<..., (StatusCode, Json<...>)>`.
2. **Filter**: `GET /tasks?done=true` — add `Query<HashMap<String,String>>` extractor, filter in `list()`.
3. **Persist**: save HashMap to `tasks.json` on every write (reuse cli-todo's `load`/`save`). What breaks with concurrent requests? (Hint: file IO inside the Mutex lock.)
4. **Logging**: print `METHOD PATH → STATUS` per request. Look at `axum::middleware` + `tower_http::trace::TraceLayer`.
5. **Real DB**: replace `Store` internals with `sqlx` + SQLite. Handlers stay almost identical — that's the point of separating routes/store.

## Limitations (on purpose)

- **Memory only** — restart wipes tasks. Persistence is exercise 3.
- **No auth** — anyone on localhost can CRUD. Real apps add tower sessions / JWT middleware.
- **`unwrap()` on lock/serve** — fine for learning; prod maps these to 500s + tracing.
