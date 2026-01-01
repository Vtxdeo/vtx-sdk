# vtx-sdk

[![Crates.io](https://img.shields.io/crates/v/vtx-sdk.svg)](https://crates.io/crates/vtx-sdk)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

**Official Rust SDK for developing VTX Project plugins.**

`vtx-sdk` provides safe, idiomatic Rust bindings for the VTX Host ABI. It abstracts away the complexity of the WebAssembly Component Model (`wit-bindgen`), allowing plugin authors to focus on business logic rather than low-level bindings.

## ✨ Features

- **Type-Safe APIs**  
  Complete Rust wrappers for all WIT import interfaces (SQL, Stream I/O, FFmpeg, Context, Event Bus).

- **Low Boilerplate**  
  `export_plugin!` and `VtxPlugin` provide default implementations for common exports
  (`migrations`, `resources`, `handle_event`, `authenticate`).

- **Database Integration**  
  SQLite helpers with automatic JSON deserialization.

- **Helper Utilities**  
  `ResponseBuilder`, `UserBuilder`, `BufferExt`, `VtxEventExt`, and a unified `VtxError` model.

## 🏗️ Architecture

Unlike traditional Wasm projects, this SDK **does not** maintain a local copy of the WIT interface definitions. Instead, it relies on the
[`vtx-protocol`](https://github.com/vtxdeo/vtx-protocol) crate as the Single Source of Truth (SSOT).

- **Build Time**  
  The `build.rs` script dynamically retrieves the WIT definition path from the `vtx-protocol`
  build dependency and injects it into the compilation process.

- **Runtime / Metadata**  
  The `WIT_DEFINITION` constant is also sourced directly from the protocol crate, ensuring
  zero divergence between the SDK and the protocol.

## 📦 Installation

Add `vtx-sdk` to your plugin’s `Cargo.toml`:

```toml
[dependencies]
vtx-sdk = "0.1.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"

[lib]
crate-type = ["cdylib"] # Required for compiling to Wasm
