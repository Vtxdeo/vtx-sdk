// ===========================
// vtx-sdk 公共预设模块 (Prelude)
// ===========================

/// 数据库操作工具
/// 包含 SQL 执行、查询及参数转换特征
pub use crate::db::{self, ToDbValue};

/// HTTP 交互工具
/// 包含请求/响应定义及响应构建器
pub use crate::http::{Request, Response, ResponseBuilder};
pub use crate::http_client::{
    request as http_request, Request as HttpClientRequest, Response as HttpClientResponse,
};

/// 鉴权与用户上下文工具及转换特征
pub use crate::auth::{AuthRequest, IntoAuthResult, UserBuilder};

/// 导出错误类型，方便插件使用 ? 操作符
pub use crate::error::{VtxError, VtxResult};

/// 核心元数据与宏
pub use crate::{export, Capabilities, HttpAllowRule, Manifest, UserContext};

/// 插件入口 Trait
/// 开发者需为结构体实现此 Trait 以作为插件实例
pub use crate::bindings::Guest as PluginTrait;

pub use crate::ffmpeg::FfmpegTask;

/// Stream I/O 工具
pub use crate::stream::{self, BufferExt, StreamBuffer};

/// 上下文工具（当前用户）
pub use crate::context::{self, CurrentUserExt, CurrentUserInfo};

/// 事件工具（事件类型与 payload 解析）
pub use crate::events::{self, PluginEvent, PluginEventContext, VtxEventExt};

/// 事件总线（发布事件）
pub use crate::event_bus;

pub use crate::export_plugin;
/// 低样板插件 Trait + 导出宏
pub use crate::plugin::VtxPlugin;
