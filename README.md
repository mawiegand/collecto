# collecto

A personal collection-management application, and a project for learning Rust
by building something real rather than toy exercises.

## Goal

Manage personal collections (items, categories, custom fields) through a
web frontend backed by a separate Rust service. The project doubles as a
guided tour through core Rust and its ecosystem: ownership and structs,
async services, IPC/pub-sub design, and WebAssembly.

## Architecture

The repo is a Cargo workspace with these crates:

- **`crates/domain`** (`collecto-domain`) — shared data types and logic for
  the collection model. No networking or storage; both backend and frontend
  depend on it directly.
- **`crates/backend`** (`collecto-backend`) — the backend service. Will use
  [dots-rust](https://github.com/pnxs/dots-rust), a type-oriented pub/sub IPC
  system, to communicate with other services through a broker (`dotsd`).
- **`crates/frontend`** (`collecto-frontend`) — the web frontend, compiled to
  WebAssembly. Talks to the backend over WebSocket/HTTP, since dots-rust's
  transport is Tokio-based and can't open raw sockets from a browser.
- **`crates/playground`** — scratch space for standalone Rust exercises,
  unrelated to the collecto app itself.

All of the above beyond the workspace skeleton is still unimplemented.

## Building

```sh
cargo build
```

Run an individual crate with `cargo run -p <crate-name>`, e.g.
`cargo run -p collecto-backend`.
