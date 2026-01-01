//! Host-side auth helpers.

use crate::bindings::vtx::api::auth_types::UserContext;
use crate::error::{VtxError, VtxResult};

/// 鉴权请求辅助类
///
/// 职责：
/// 封装原始 Header 列表，提供便捷的 Token 提取与校验方法。
pub struct AuthRequest<'a> {
    headers: &'a [(String, String)],
}

impl<'a> AuthRequest<'a> {
    /// 实例化 AuthRequest
    pub fn new(headers: &'a [(String, String)]) -> Self {
        Self { headers }
    }

    /// 获取 Header 值 (Case-insensitive)
    pub fn header(&self, key: &str) -> Option<&str> {
        let search_key = key.to_lowercase();
        for (k, v) in self.headers {
            if k.to_lowercase() == search_key {
                return Some(v.as_str());
            }
        }
        None
    }

    /// 获取必需的 Header 值
    ///
    /// 行为：
    /// 若 Header 不存在，返回 `AuthDenied(401)` 错误。
    pub fn require_header(&self, key: &str) -> VtxResult<&str> {
        self.header(key).ok_or_else(|| {
            // 提示：具体缺失哪个 Header 的信息在转换为 u16 时会丢失，
            // 但在调试阶段或后续日志扩展中可能有用。
            VtxError::AuthDenied(401)
        })
    }

    /// 提取 Bearer Token
    ///
    /// 格式支持：`Authorization: Bearer <token>` (忽略 Bearer 大小写)。
    pub fn bearer_token(&self) -> Option<&str> {
        let val = self.header("Authorization")?;
        if val.starts_with("Bearer ") || val.starts_with("bearer ") {
            Some(&val[7..])
        } else {
            None
        }
    }

    /// 获取必需的 Bearer Token
    ///
    /// 行为：
    /// 若 Authorization 头缺失或格式不正确，返回 `AuthDenied(401)`。
    pub fn require_bearer_token(&self) -> VtxResult<&str> {
        self.bearer_token().ok_or_else(|| VtxError::AuthDenied(401))
    }

    /// 提取 Basic Auth 凭证
    pub fn basic_auth(&self) -> Option<&str> {
        let val = self.header("Authorization")?;
        if val.starts_with("Basic ") || val.starts_with("basic ") {
            Some(&val[6..])
        } else {
            None
        }
    }
}

/// 用户上下文构建器 (Builder Pattern)
///
/// 职责：
/// 构造 `UserContext` 对象，支持链式调用。
pub struct UserBuilder {
    user_id: String,
    username: String,
    groups: Vec<String>,
    metadata: serde_json::Map<String, serde_json::Value>,
}

impl UserBuilder {
    /// 初始化构建器
    ///
    /// 必需参数：用户 ID 和 用户名。
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            user_id: id.into(),
            username: name.into(),
            groups: Vec::new(),
            metadata: serde_json::Map::new(),
        }
    }

    /// 添加用户所属组/角色
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.groups.push(group.into());
        self
    }

    /// 添加元数据键值对
    ///
    /// 若 `value` 序列化失败，该字段将被静默忽略。
    pub fn meta<V: serde::Serialize>(mut self, key: &str, value: V) -> Self {
        if let Ok(val) = serde_json::to_value(value) {
            self.metadata.insert(key.to_string(), val);
        }
        self
    }

    /// 构建 UserContext
    ///
    /// 结果包含序列化后的 metadata JSON 字符串。
    pub fn build(self) -> UserContext {
        UserContext {
            user_id: self.user_id,
            username: self.username,
            groups: self.groups,
            metadata: serde_json::to_string(&self.metadata).unwrap_or_else(|_| "{}".to_string()),
        }
    }
}

/// 鉴权结果转换扩展特征
///
/// 职责：
/// 将 SDK 标准的 `VtxResult<UserContext>` 转换为 WIT 接口要求的 `Result<UserContext, u16>`。
/// 这允许开发者在 `authenticate` 实现中统一使用 `?` 操作符处理 DB 或逻辑错误。
pub trait IntoAuthResult {
    fn into_auth_result(self) -> Result<UserContext, u16>;
}

impl IntoAuthResult for VtxResult<UserContext> {
    fn into_auth_result(self) -> Result<UserContext, u16> {
        match self {
            Ok(ctx) => Ok(ctx),
            Err(e) => {
                // 错误降级策略：将丰富的错误类型映射为 HTTP 状态码
                let status_code = match e {
                    VtxError::AuthDenied(code) => code,
                    VtxError::PermissionDenied(_) => 403,
                    VtxError::NotFound(_) => 404,
                    // 数据库错误、序列化错误或内部错误，统一视为 500
                    VtxError::DatabaseError(_)
                    | VtxError::SerializationError(_)
                    | VtxError::Internal(_) => 500,
                };
                Err(status_code)
            }
        }
    }
}
