// ===========================
// vtx-sdk 公共导出模块 (prelude)
// 提供给插件开发者统一使用
// ===========================

/// 数据库抽象层（参数封装、类型转换）
pub use crate::db::{self, ToDbValue};

/// HTTP 请求/响应构建工具
pub use crate::http::{Request, Response, ResponseBuilder};

/// 鉴权结构与用户上下文构建器
pub use crate::auth::{AuthRequest, UserBuilder};

/// 插件清单、用户上下文、导出宏
pub use crate::{Manifest, UserContext, export};

/// 插件执行入口 trait（WIT 接口映射）
pub use crate::bindings::Guest as PluginTrait;
