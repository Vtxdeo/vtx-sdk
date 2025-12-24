/// WIT 接口绑定模块（私有）
///
/// 通过 `wit_bindgen` 宏将 `wit/vtx.wit` 文件中定义的接口自动生成 Rust 类型绑定。
/// 此模块为内部绑定实现，不应被外部直接访问，应通过 `prelude` 或显式导出的类型使用。
pub mod bindings {
    wit_bindgen::generate!({
        world: "plugin",
        path: "wit",
        pub_export_macro: true,
    });
}

// =====================
// 模块定义
// =====================

/// 数据库工具模块（迁移、结构、连接封装）
pub mod db;

/// HTTP 请求 / 响应结构与构造器
pub mod http;

/// 用户身份认证与上下文结构工具
pub mod auth;

/// 错误类型定义与统一错误处理
pub mod error;

/// 通用预导入模块（导出常用类型/宏/辅助函数）
pub mod prelude;

// =====================
// 类型导出（供外部模块使用）
// =====================

/// 导出插件实现接口定义（`export!(...)`）
pub use bindings::export;

/// 用户上下文结构，常用于授权接口
pub use bindings::vtx::api::auth_types::UserContext;

/// 插件清单类型，用于插件元数据管理
pub use bindings::vtx::api::types::Manifest;
