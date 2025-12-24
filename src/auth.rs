use crate::bindings::vtx::api::auth_types::UserContext;

/// 鉴权请求辅助类
///
/// 职责：
/// 封装原始 Header 列表，提供便捷的 Token 提取方法。
pub struct AuthRequest<'a> {
    headers: &'a [(String, String)],
}

impl<'a> AuthRequest<'a> {
    /// 实例化 AuthRequest
    pub fn new(headers: &'a [(String, String)]) -> Self {
        Self { headers }
    }

    /// 获取 Header 值
    ///
    /// 行为：
    /// 执行大小写不敏感 (Case-insensitive) 的键查找。
    /// 复杂度：O(N)，N 为 Header 数量。
    pub fn header(&self, key: &str) -> Option<&str> {
        let search_key = key.to_lowercase();
        for (k, v) in self.headers {
            if k.to_lowercase() == search_key {
                return Some(v.as_str());
            }
        }
        None
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

    /// 提取 Basic Auth 凭证
    ///
    /// 格式支持：`Authorization: Basic <base64>` (忽略 Basic 大小写)。
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
/// 构造 `UserContext` 对象，通常在 `authenticate` 接口实现中使用。
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
