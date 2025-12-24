use serde::{Deserialize, Serialize};
use std::fmt;

/// 插件运行时错误类型（统一错误模型）
///
/// 定义插件在运行过程中可能出现的所有已知错误。
/// 所有错误均支持序列化，可用于 HTTP 返回或日志透传。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VtxError {
    /// 底层数据库错误（如 SQL 执行失败、约束冲突等）
    DatabaseError(String),

    /// 序列化或反序列化失败（如 JSON 格式不匹配）
    SerializationError(String),

    /// 身份验证失败（带建议返回的 HTTP 状态码，如 401 / 403）
    AuthDenied(u16),

    /// 权限不足（如在只读环境尝试执行写操作）
    PermissionDenied(String),

    /// 资源不存在（如文件、视频、用户未找到等）
    NotFound(String),

    /// 插件内部逻辑错误（兜底类型）
    Internal(String),
}

impl fmt::Display for VtxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VtxError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            VtxError::SerializationError(msg) => write!(f, "Data serialization error: {}", msg),
            VtxError::AuthDenied(code) => write!(f, "Authentication denied (Code: {})", code),
            VtxError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            VtxError::NotFound(msg) => write!(f, "Resource not found: {}", msg),
            VtxError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for VtxError {}

/// 插件标准结果类型别名
///
/// 推荐用于所有返回 VtxError 的接口中，确保错误链统一。
pub type VtxResult<T> = Result<T, VtxError>;
