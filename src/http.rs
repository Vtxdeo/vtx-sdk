use crate::bindings::vtx::api::{
    stream_io,
    types::{HttpResponse, HttpRequest},
};

/// 类型别名：插件侧统一使用的请求/响应结构
pub type Request = HttpRequest;
pub type Response = HttpResponse;

/// 响应构造器（用于生成标准 HTTP 响应体）
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// 返回 200 OK，Body 为 JSON 序列化结果
    pub fn json<T: serde::Serialize>(data: &T) -> Response {
        let json_bytes = serde_json::to_vec(data).unwrap_or_else(|_| b"[]".to_vec());

        let buffer = stream_io::create_memory_buffer(&json_bytes);
        HttpResponse {
            status: 200,
            body: Some(buffer),
        }
    }

    /// 返回 200 OK，Body 为已注册资源（文件流）
    /// 若找不到资源，返回 404
    pub fn file(uuid: &str) -> Response {
        match stream_io::open_file(uuid) {
            Ok(buffer) => HttpResponse {
                status: 200,
                body: Some(buffer),
            },
            Err(_) => HttpResponse {
                status: 404,
                body: None,
            },
        }
    }

    /// 返回指定状态码（无内容）
    pub fn status(code: u16) -> Response {
        HttpResponse {
            status: code,
            body: None,
        }
    }

    /// 返回 404 Not Found
    pub fn not_found() -> Response {
        Self::status(404)
    }
}
