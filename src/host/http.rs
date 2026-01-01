//! Host-side HTTP request/response helpers.

use crate::bindings::vtx::api::{
    stream_io,
    types::{HttpRequest, HttpResponse},
};
use crate::error::VtxError;

/// 标准化请求与响应类型别名（与 WIT 接口对齐）
pub type Request = HttpRequest;
pub type Response = HttpResponse;

/// HTTP 响应构造器（适用于插件运行时）
///
/// 提供以下构造能力：
/// - 成功响应（JSON）
/// - 错误响应（自动映射状态码与结构体）
/// - 文件流响应（基于宿主 UUID 打开文件）
/// - 状态码响应（纯状态码，无 body）
///
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// 构造 JSON 响应（200 OK）
    ///
    /// ⚠️ 若序列化失败，返回 `[]` 作为兜底内容，**不表示逻辑成功**。
    pub fn json<T: serde::Serialize>(data: &T) -> Response {
        let json_bytes = serde_json::to_vec(data).unwrap_or_else(|_| b"[]".to_vec());

        // 使用宿主提供的内存缓冲区封装 JSON 数据
        let buffer = stream_io::create_memory_buffer(&json_bytes);
        HttpResponse {
            status: 200,
            body: Some(buffer),
        }
    }

    /// 构造错误响应（根据错误类型自动映射 HTTP 状态码）
    ///
    /// - `AuthDenied(code)` → `code`（401 / 403）
    /// - `NotFound(_)` → 404
    /// - `PermissionDenied(_)` → 403
    /// - `SerializationError(_)` → 400
    /// - `DatabaseError(_)`, `Internal(_)` → 500
    ///
    /// 返回结构：
    /// ```json
    /// {
    ///   "success": false,
    ///   "error": true,
    ///   "code": 403,
    ///   "type": "PermissionDenied",
    ///   "message": "You are not allowed to access this resource"
    /// }
    /// ```
    pub fn error(err: VtxError) -> Response {
        let (status, message) = match &err {
            VtxError::AuthDenied(code) => (*code, format!("Authentication failed: {}", err)),
            VtxError::NotFound(msg) => (404, msg.clone()),
            VtxError::PermissionDenied(msg) => (403, msg.clone()),
            VtxError::SerializationError(msg) => (400, format!("Bad Request: {}", msg)),
            VtxError::DatabaseError(msg) => (500, format!("Database Error: {}", msg)),
            VtxError::Internal(msg) => (500, format!("Internal Error: {}", msg)),
        };

        let error_body = serde_json::json!({
            "success": false,                  // 统一布尔失败标识
            "error": true,                     // 标识为错误响应
            "code": status,                    // 映射的 HTTP 状态码
            "type": format!("{:?}", err),      // 错误类型（调试使用）
            "message": message                 // 错误消息（用户可见）
        });

        let mut resp = Self::json(&error_body);
        resp.status = status;
        resp
    }

    /// 构造文件流响应（通过宿主接口按 UUID 打开）
    ///
    /// - 成功：200 + 文件内容流
    /// - 失败：返回 404 JSON 错误响应
    pub fn file(uuid: &str) -> Response {
        match stream_io::open_file(uuid) {
            Ok(buffer) => HttpResponse {
                status: 200,
                body: Some(buffer),
            },
            Err(e) => Self::error(VtxError::NotFound(format!("File UUID not found: {}", e))),
        }
    }

    /// 构造纯状态码响应（无 body）
    ///
    /// 用于如：204 No Content、403 Forbidden 等响应场景
    pub fn status(code: u16) -> Response {
        HttpResponse {
            status: code,
            body: None,
        }
    }

    /// 构造标准 404 Not Found 响应（无正文）
    pub fn not_found() -> Response {
        Self::status(404)
    }
}
