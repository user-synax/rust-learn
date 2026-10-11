// main.rs — STEP 4: GLUE. State → Router → Serve.
//
// WHAT A BACKEND DOES AT STARTUP:
//   1. Build shared state (Store)
//   2. Map URL + METHOD → handler fn (Router)
//   3. Bind a TCP port and serve forever
//
// JS (Express) equivalent:
//   const app = express(); app.use(express.json());
//   app.get("/health", health); app.get("/tasks", listTasks); ...
//   app.listen(3000, () => console.log("on 3000"));
//
// Rust version below does the same 3 things, with two differences:
// - `#[tokio::main]` — entry point for ASYNC code (see WHY below).
// - State is EXPLICIT and TYPE-CHECKED: forget `.with_state()` and it won't compile.

mod models;
mod routes;
mod store;

use std::sync::Arc;

use axum::{routing::get, Router};
use store::Store;

/// WHY `#[tokio::main]` + `async fn main`?
/// Handlers are `async` (they wait on network IO). Async code needs a
/// RUNTIME to schedule it. `tokio` is that runtime (like Node's event loop,
/// but multi-threaded). The attribute expands to: build runtime → block_on(main()).
#[tokio::main]
async fn main() {
    // STEP 4a: ONE store for the whole server, wrapped in Arc for sharing.
    // `Arc::new(...)` = atomic ref-counted pointer. Each request handler
    // gets a cheap clone pointing at the SAME Store (see store.rs).
    let store: store::SharedStore = Arc::new(Store::new());

    // STEP 4b: URL table. `get(handler)` = GET, `.post(h)` = POST on same path.
    // `.route("/tasks/{id}", ...)` — `{id}` is axum 0.8 syntax for a path param.
    // It lands in handlers as `Path<u64>`. Non-numeric id → 422 automatically.
    let app: Router = Router::new()
        .route("/health", get(routes::health))
        .route("/tasks", get(routes::list_tasks).post(routes::create_task))
        .route(
            "/tasks/{id}",
            get(routes::get_task)
                .put(routes::update_task)
                .delete(routes::delete_task),
        )
        .with_state(store); // inject SharedStore into every handler taking `State(...)`

    // STEP 4c: Bind + serve forever.
    // `127.0.0.1:3000` = localhost only. Use `0.0.0.0:3000` to expose on your LAN/docker.
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("todo-api listening on http://{addr}");
    println!("try: curl http://{addr}/health");

    // This future never resolves — it handles requests until Ctrl+C.
    // `.unwrap()` on serve: if the socket dies, crash loudly (correct for learning).
    axum::serve(listener, app).await.unwrap();
}
