/// WIT 绑定生成模块
///
/// 职责：
/// 根据 `wit/vtx.wit` 定义生成 Rust 绑定代码。
/// 这是一个私有模块，外部通过 `prelude` 或其他模块访问功能。
mod bindings {
    wit_bindgen::generate!({
        world: "plugin",
        path: "wit",
        pub_export_macro: true,
    });
}

// 模块定义
pub mod db;

// 子模块：HTTP 请求/响应结构与构造器
pub mod http;

// 子模块：用户身份鉴权与上下文工具
pub mod auth;

// SDK 统一入口，聚合导出（给插件调用者使用）
pub mod prelude;

// 显式导出核心类型，供外部引用
pub use bindings::export;
pub use bindings::vtx::api::auth_types::UserContext;
pub use bindings::vtx::api::types::Manifest;
