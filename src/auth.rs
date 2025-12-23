use crate::bindings::vtx::api::auth_types::UserContext;

/// 提供对请求头的便捷访问与解析
pub struct AuthRequest<'a> {
    headers: &'a [(String, String)],
}

impl<'a> AuthRequest<'a> {
    /// 创建新的 AuthRequest 实例
    pub fn new(headers: &'a [(String, String)]) -> Self {
        Self { headers }
    }

    /// 获取指定 Header 的值（不区分大小写）
    ///
    /// 返回匹配到的第一个值，若未找到则返回 None。
    pub fn header(&self, key: &str) -> Option<&str> {
        let search_key = key.to_lowercase();
        for (k, v) in self.headers {
            if k.to_lowercase() == search_key {
                return Some(v.as_str());
            }
        }
        None
    }

    /// 提取 Bearer Token（来自 Authorization 头）
    ///
    /// 解析形如 "Authorization: Bearer <token>" 的值。
    pub fn bearer_token(&self) -> Option<&str> {
        let val = self.header("Authorization")?;
        if val.starts_with("Bearer ") || val.starts_with("bearer ") {
            Some(&val[7..])
        } else {
            None
        }
    }

    /// 提取 Basic Auth（来自 Authorization 头）
    ///
    /// 解析形如 "Authorization: Basic <base64>" 的值。
    pub fn basic_auth(&self) -> Option<&str> {
        let val = self.header("Authorization")?;
        if val.starts_with("Basic ") || val.starts_with("basic ") {
            Some(&val[6..])
        } else {
            None
        }
    }
}

/// 构建 UserContext 的辅助工具（Fluent Builder 模式）
pub struct UserBuilder {
    user_id: String,
    username: String,
    groups: Vec<String>,
    metadata: serde_json::Map<String, serde_json::Value>,
}

impl UserBuilder {
    /// 创建新的 UserBuilder 实例
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            user_id: id.into(),
            username: name.into(),
            groups: Vec::new(),
            metadata: serde_json::Map::new(),
        }
    }

    /// 添加用户所属组（权限角色）
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.groups.push(group.into());
        self
    }

    /// 添加用户元数据（JSON 格式的键值对）
    pub fn meta<V: serde::Serialize>(mut self, key: &str, value: V) -> Self {
        if let Ok(val) = serde_json::to_value(value) {
            self.metadata.insert(key.to_string(), val);
        }
        self
    }

    /// 构建最终的 UserContext 对象
    pub fn build(self) -> UserContext {
        UserContext {
            user_id: self.user_id,
            username: self.username,
            groups: self.groups,
            metadata: serde_json::to_string(&self.metadata).unwrap_or_else(|_| "{}".to_string()),
        }
    }
}
