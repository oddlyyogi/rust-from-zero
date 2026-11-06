# 34 - Build a Web API with axum

How do you build a web API in Rust? The front counter of a parcel depot explains routers and handlers. We start a real server with axum and tokio, read values from the address, answer with JSON, share state between requests, create parcels with POST, return not found with Option and Result, and see how axum answers bad requests for us.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
fn main() {
}
```

Compiles and runs:

```rust
use axum::{Router, extract::Path, routing::get};
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Harbor Parcels is open"
}

async fn track(Path(id): Path<u32>) -> String {
    format!("Parcel {id} is on its way")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/parcels/{id}", get(track));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Sources

- axum docs, routing, extractors, order of extractors, state: https://docs.rs/axum/latest/axum/
- axum extract module, the body extractor must be last: https://docs.rs/axum/latest/axum/extract/index.html
- axum Json rejections 415, 422 and 400: https://docs.rs/axum/latest/axum/struct.Json.html
- Tokio tutorial on std Mutex and holding locks across await: https://tokio.rs/tokio/tutorial/shared-state
- HTTP status codes: https://www.rfc-editor.org/rfc/rfc9110
