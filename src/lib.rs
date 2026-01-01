/// WIT 接口绑定模块（私有）
///
/// # 说明
/// 该模块负责生成和管理与 VTX Protocol 定义的接口绑定。
/// 它不再依赖本地文件，而是直接使用 `vtx-protocol` 提供的单一事实来源。
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// =====================
// 模块定义
// =====================

/// 宿主 ABI 封装（按领域拆分，便于维护）
pub mod host;

/// 兼容性导出：保持 `vtx_sdk::db/...` 等路径不变
pub use host::{auth, context, db, event_bus, events, ffmpeg, http, stream};

/// 错误类型定义与统一错误处理
pub mod error;

/// 通用预导入模块（导出常用类型/宏/辅助函数）
pub mod prelude;

/// 更低样板的插件导出适配
pub mod plugin;

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

/// 当前用户信息（来自宿主上下文）
pub use bindings::vtx::api::auth_types::CurrentUser;

// =====================
// 元数据导出 (仅在 meta 特性开启时可用)
// =====================

/// 暴露 SDK 使用的 WIT 接口定义内容
/// 直接复用 vtx-protocol crate 中的常量，零运行时开销
#[cfg(feature = "meta")]
pub const WIT_DEFINITION: &str = vtx_protocol::WIT_CONTENT;

/// SDK 版本号
#[cfg(feature = "meta")]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
