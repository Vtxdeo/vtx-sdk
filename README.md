# vtx-sdk

[![Crates.io](https://img.shields.io/crates/v/vtx-sdk.svg)](https://crates.io/crates/vtx-sdk)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

**Official Rust SDK for developing VTX Project plugins.**

`vtx-sdk` provides safe, idiomatic Rust bindings for the VTX Host ABI. It abstracts away the complexity of the WebAssembly Component Model (`wit-bindgen`), allowing you to focus on building powerful video processing logic.

## ✨ Features

* **Type-Safe APIs**: Full Rust type support for database queries, HTTP handling, and file I/O.
* **Zero Boilerplate**: Use the `vtx_sdk::export!` macro to generate all necessary Wasm component exports.
* **Database Integration**: Built-in SQLite helpers with automatic JSON serialization/deserialization.
* **Helper Utilities**:
    * `ResponseBuilder` for fluent HTTP responses.
    * `UserBuilder` for easy auth context management.
    * `VtxError` for unified error handling.

## 🏗️ Architecture

Unlike traditional Wasm projects, this SDK **does not** maintain a local copy of the WIT interface definitions. Instead, it relies on the [vtx-protocol](https://github.com/vtxdeo/vtx-protocol) crate as the Single Source of Truth (SSOT).

* **Build Time**: The `build.rs` script dynamically fetches the WIT definition path from the `vtx-protocol` build dependency and injects it into the compilation process.
* **Runtime/Metadata**: The `WIT_DEFINITION` constant is also sourced directly from the protocol crate, ensuring zero divergence between the SDK and the Protocol.

## 📦 Installation

Add `vtx-sdk` to your plugin's `Cargo.toml`:

```toml
[dependencies]
vtx-sdk = "0.1.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"

[lib]
crate-type = ["cdylib"] # Essential for compiling to Wasm
````

## 🚀 Quick Start

Here is a minimal example of a VTX plugin:

```rust
use vtx_sdk::prelude::*;

// 1. Define your plugin structure
struct MyPlugin;

// 2. Export the component world
vtx_sdk::export!(MyPlugin);

// 3. Implement the Plugin trait
impl Plugin for MyPlugin {
    fn handle(req: HttpRequest) -> Result<HttpResponse, VtxError> {
        // Log to the host console
        println!("Received request: {} {}", req.method, req.path);

        // Return a JSON response
        ResponseBuilder::ok()
            .json(&serde_json::json!({
                "message": "Hello from VTX Plugin!",
                "path": req.path
            }))
            .build()
    }

    fn get_manifest() -> Result<Manifest, VtxError> {
        Ok(Manifest {
            id: "com.example.hello".to_string(),
            version: "0.1.0".to_string(),
            name: "Hello World Plugin".to_string(),
            description: "A simple example plugin".to_string(),
            entrypoint: "/hello".to_string(),
        })
    }

    // Optional: Declare database tables
    fn get_resources() -> Result<Vec<String>, VtxError> {
        Ok(vec![])
    }

    // Optional: SQL migrations
    fn get_migrations() -> Result<Vec<String>, VtxError> {
        Ok(vec![])
    }
}
```

## 🛠️ Build & Deploy

Plugins must be compiled to the `wasm32-wasip1` target.

### Prerequisites

```bash
rustup target add wasm32-wasip1
```

### Building with vtx-cli (Recommended)

The official CLI handles building, stripping, and packaging (`.vtx`) automatically.

```bash
vtx-cli build --package vtx-plugin-example
```

### Manual Build

If you are not using the CLI, you can build the raw Wasm file:

```bash
cargo build --release --target wasm32-wasip1
```

*Note: Raw Wasm files may need to be adapted using `wasm-tools` to work with the VTX runtime if not using `vtx-cli`.*

## 📚 Modules Overview

* **`db`**: Helpers for executing SQL queries and managing database transactions.

    * Example: `db::query("SELECT * FROM videos")`

* **`http`**: HTTP request/response types and builders.

* **`auth`**: Utilities for parsing headers and constructing `UserContext`.

* **`error`**: The `VtxError` enum maps internal errors to appropriate HTTP status codes (e.g., `VtxError::Unauthorized` -> 401).

## 📄 License

This project is licensed under the [Apache 2.0 License](https://www.google.com/search?q=LICENSE).
