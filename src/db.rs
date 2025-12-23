use crate::bindings::vtx::api::sql::{self, DbValue};
use serde::de::DeserializeOwned;

/// 支持转换为 DbValue 的 Rust 类型（SQL 参数映射）
pub trait ToDbValue {
    fn to_db_value(&self) -> DbValue;
}

// 实现常用类型的参数转换逻辑
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

/// 执行 INSERT / UPDATE / DELETE 等无结果集语句
///
/// - 返回影响的行数（u64）
/// - 错误时返回错误信息字符串
pub fn execute(sql: &str, params: &[&dyn ToDbValue]) -> Result<u64, String> {
    let wit_params: Vec<DbValue> = params.iter().map(|p| p.to_db_value()).collect();
    sql::execute(sql, &wit_params)
}

/// 执行 SELECT 查询，并自动反序列化为结构体列表
///
/// - 通过宿主返回的 JSON 字符串解析成目标类型 Vec<T>
/// - 要求目标类型实现 serde::DeserializeOwned
pub fn query<T: DeserializeOwned>(sql: &str, params: &[&dyn ToDbValue]) -> Result<Vec<T>, String> {
    let wit_params: Vec<DbValue> = params.iter().map(|p| p.to_db_value()).collect();

    // 调用宿主接口获取 JSON 字符串结果
    let json_str = sql::query_json(sql, &wit_params)?;

    // 使用 serde_json 进行类型转换
    serde_json::from_str(&json_str).map_err(|e| format!("JSON Parse Error: {}", e))
}
