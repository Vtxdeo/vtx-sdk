//! Host-side SQL helpers.

use crate::bindings::vtx::api::sql::{self, DbValue};
use crate::error::{VtxError, VtxResult};
use serde::de::DeserializeOwned;

/// Trait：用于将 Rust 类型转换为 WIT 定义的 `DbValue`
///
/// 适用于数据库跨边界调用参数传递。
pub trait ToDbValue {
    fn to_db_value(&self) -> DbValue;
}

// --- 基本类型到 DbValue 的映射实现 ---

impl ToDbValue for String {
    fn to_db_value(&self) -> DbValue {
        DbValue::Text(self.clone())
    }
}

impl ToDbValue for &str {
    fn to_db_value(&self) -> DbValue {
        DbValue::Text(self.to_string())
    }
}

impl ToDbValue for i64 {
    fn to_db_value(&self) -> DbValue {
        DbValue::Integer(*self)
    }
}

impl ToDbValue for i32 {
    fn to_db_value(&self) -> DbValue {
        DbValue::Integer(*self as i64)
    }
}

impl ToDbValue for f64 {
    fn to_db_value(&self) -> DbValue {
        DbValue::Real(*self)
    }
}

impl ToDbValue for f32 {
    fn to_db_value(&self) -> DbValue {
        DbValue::Real(*self as f64)
    }
}

impl ToDbValue for u64 {
    fn to_db_value(&self) -> DbValue {
        DbValue::Integer(*self as i64)
    }
}

impl ToDbValue for u32 {
    fn to_db_value(&self) -> DbValue {
        DbValue::Integer(*self as i64)
    }
}

impl ToDbValue for bool {
    fn to_db_value(&self) -> DbValue {
        DbValue::Integer(if *self { 1 } else { 0 })
    }
}

impl ToDbValue for () {
    fn to_db_value(&self) -> DbValue {
        DbValue::NullVal
    }
}

impl<T: ToDbValue> ToDbValue for Option<T> {
    fn to_db_value(&self) -> DbValue {
        match self {
            Some(v) => v.to_db_value(),
            None => DbValue::NullVal,
        }
    }
}

/// 执行非查询类 SQL（INSERT / UPDATE / DELETE）
///
/// # Parameters
/// - `sql`: SQL 原始字符串，支持 `?` 占位符
/// - `params`: 参数数组，元素需实现 `ToDbValue`
///
/// # Returns
/// - 成功：返回影响行数
/// - 失败：映射为 `VtxError::DatabaseError`
///
/// ⚠️ 注意：Restricted 安全策略下禁止调用该接口
pub fn execute(sql: &str, params: &[&dyn ToDbValue]) -> VtxResult<u64> {
    let wit_params: Vec<DbValue> = params.iter().map(|p| p.to_db_value()).collect();

    sql::execute(sql, &wit_params).map_err(|e| {
        if e.to_lowercase().contains("permission denied") {
            VtxError::PermissionDenied(e)
        } else {
            VtxError::DatabaseError(e)
        }
    })
}

/// 执行查询类 SQL（SELECT）并反序列化为目标类型列表
///
/// # Parameters
/// - `sql`: SQL 字符串（支持 ? 占位符）
/// - `params`: 参数数组（实现 `ToDbValue`）
///
/// # Returns
/// - 成功：反序列化后的结果集合
/// - 失败：`DatabaseError` 或 `SerializationError`
///
/// # Notes
/// - 宿主接口返回的是 JSON 字符串
/// - 为保证性能，建议单次返回控制在 1MB 内（可通过 LIMIT 分页）
/// - 泛型 `T` 必须实现 `DeserializeOwned`（无需生命周期）
///
pub fn query<T: DeserializeOwned>(sql: &str, params: &[&dyn ToDbValue]) -> VtxResult<Vec<T>> {
    let wit_params: Vec<DbValue> = params.iter().map(|p| p.to_db_value()).collect();

    let json_str = sql::query_json(sql, &wit_params).map_err(|e| {
        if e.to_lowercase().contains("permission denied") {
            VtxError::PermissionDenied(e)
        } else {
            VtxError::DatabaseError(e)
        }
    })?;

    serde_json::from_str(&json_str).map_err(|e| VtxError::SerializationError(e.to_string()))
}
