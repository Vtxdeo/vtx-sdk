use crate::bindings::vtx::api::sql::{self, DbValue};
use serde::de::DeserializeOwned;

/// 数据库参数转换特征
///
/// 职责：
/// 将 Rust 原生类型转换为 WIT 定义的 `DbValue` 变体，以便跨边界传递。
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

/// 执行非查询语句 (INSERT/UPDATE/DELETE)
///
/// 参数：
/// - `sql`: 原始 SQL 语句，支持 `?` 占位符。
/// - `params`: 参数列表，需实现 `ToDbValue`。
///
/// 返回值：
/// - 成功：受影响的行数 (rows_affected)。
/// - 失败：错误描述字符串。
pub fn execute(sql: &str, params: &[&dyn ToDbValue]) -> Result<u64, String> {
    let wit_params: Vec<DbValue> = params.iter().map(|p| p.to_db_value()).collect();
    sql::execute(sql, &wit_params)
}

/// 执行查询语句 (SELECT)
///
/// 行为：
/// 1. 调用宿主接口执行 SQL。
/// 2. 宿主将结果集序列化为 JSON 字符串返回。
/// 3. 插件侧反序列化为泛型 `Vec<T>`。
///
/// 性能边界：
/// 由于涉及 JSON 序列化/反序列化及内存拷贝，此方法不适合处理
/// 超过 1MB 的大数据集。建议在 SQL 中使用 `LIMIT` 进行分页。
pub fn query<T: DeserializeOwned>(sql: &str, params: &[&dyn ToDbValue]) -> Result<Vec<T>, String> {
    let wit_params: Vec<DbValue> = params.iter().map(|p| p.to_db_value()).collect();

    // 跨边界调用：获取 JSON 结果
    let json_str = sql::query_json(sql, &wit_params)?;

    // 本地反序列化
    serde_json::from_str(&json_str).map_err(|e| format!("JSON Parse Error: {}", e))
}
