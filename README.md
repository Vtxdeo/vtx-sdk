# vtx-sdk

[![Crates.io](https://img.shields.io/crates/v/vtx-sdk.svg)](https://crates.io/crates/vtx-sdk)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

**Official Rust SDK for developing VTX Project plugins.**

`vtx-sdk` provides safe, idiomatic Rust bindings for the VTX Host ABI. It abstracts away the complexity of the WebAssembly Component Model (`wit-bindgen`), allowing you to focus on building powerful video processing logic.

## ✨ Features

* **Type-Safe APIs**: Rust 封装覆盖全部 WIT import 接口（SQL / Stream I/O / FFmpeg / Context / Event Bus）。
* **Low Boilerplate**: `export_plugin!` + `VtxPlugin` 提供默认实现（migrations/resources/handle_event/authenticate）。
* **Database Integration**: SQLite helpers + JSON 自动反序列化。
* **Helper Utilities**: `ResponseBuilder` / `UserBuilder` / `BufferExt` / `VtxEventExt` / `VtxError`。

## 🏗️ Architecture

Unlike traditional Wasm projects, this SDK **does not** maintain a local copy of the WIT interface definitions. Instead, it relies on the [vtx-protocol](https://github.com/vtxdeo/vtx-protocol) crate as the Single Source of Truth (SSOT).

* **Build Time**: The `build.rs` script dynamically fetches the WIT definition path from the `vtx-protocol` build dependency and injects it into the compilation process.
* **Runtime/Metadata**: The `WIT_DEFINITION` constant is also sourced directly from the protocol crate, ensuring zero divergence between the SDK and the Protocol.

## 📦 Installation

Add `vtx-sdk` to your plugin's `Cargo.toml`:

```toml
[dependencies]
vtx-sdk = "0.1.4"
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

struct MyPlugin;

impl VtxPlugin for MyPlugin {
    fn handle(req: Request) -> VtxResult<Response> {
        println!("Received request: {} {}", req.method, req.path);
        Ok(ResponseBuilder::json(&serde_json::json!({
            "message": "Hello from VTX Plugin!",
            "path": req.path,
            "query": req.query
        })))
    }

    fn get_manifest() -> Manifest {
        Manifest {
            id: "com.example.hello".to_string(),
            name: "Hello World Plugin".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "A simple example plugin".to_string(),
            entrypoint: "/hello".to_string(),
        }
    }
}

export_plugin!(MyPlugin);
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

* **`stream`**: File open + `Buffer` read/write helpers (`BufferExt`).

* **`context`**: Current user helpers (`current_user()`).

* **`events`**: Event payload parsing helpers (`VtxEventExt`).

* **`event_bus`**: Publish events to the host (`publish_json`).

* **`plugin`**: `VtxPlugin` + `export_plugin!` (low boilerplate exports).

* **`error`**: Unified error model (`VtxError::AuthDenied(401)`, `VtxError::PermissionDenied(...)`, ...).

## 📄 License

This project is licensed under the [Apache 2.0 License](https://www.google.com/search?q=LICENSE).
