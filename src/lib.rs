/// WIT 接口绑定模块（私有）
///
/// # 说明
/// 该模块负责生成和管理与 `wit/vtx.wit` 文件中定义的接口绑定。
/// 通过使用 `wit_bindgen` 宏，自动将 `wit/vtx.wit` 文件中的接口转换为 Rust 类型。
///
/// **注意**：该模块依赖根目录下的 `wit/` 文件夹，在发布时需要确保 `Cargo.toml` 的 `include` 字段包含该路径。
/// 在构建过程中，`wit_bindgen` 宏会根据 `wit/vtx.wit` 文件生成绑定代码。
///
/// 该模块仅用于内部实现，不应直接在外部访问。外部应该通过 `prelude` 或显式导出的类型来使用。
pub mod bindings {
    // 使用 wit_bindgen 宏生成 WIT 接口的 Rust 类型绑定
    wit_bindgen::generate!({
        world: "plugin",  // 定义插件名称
        // `path` 是相对于 `Cargo.toml` 的路径，指定 WIT 文件所在的位置
        // 在作为 crate 依赖时，cargo 会解压源码，此路径仍然有效
        path: "wit",
        pub_export_macro: true,  // 导出宏，以供外部调用
        default_bindings_module: "vtx_sdk::bindings",  // 默认绑定模块路径
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
/// 提供给外部使用插件的接口。
pub use bindings::export;

/// 用户上下文结构，常用于授权接口
/// `UserContext` 类型常用于用户身份认证和权限检查的上下文数据。
pub use bindings::vtx::api::auth_types::UserContext;

/// 插件清单类型，用于插件元数据管理
/// `Manifest` 类型用于表示插件的元数据和描述信息。
pub use bindings::vtx::api::types::Manifest;

// =====================
// 元数据导出 (仅在 meta 特性开启时可用)
// =====================

/// 暴露 SDK 内置的 WIT 接口定义文件内容
/// CLI 工具可以使用此常量来验证插件是否符合当前版本的契约
#[cfg(feature = "meta")]
pub const WIT_DEFINITION: &str = include_str!("../wit/vtx.wit");

/// SDK 版本号
#[cfg(feature = "meta")]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
