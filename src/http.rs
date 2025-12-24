use crate::bindings::vtx::api::{
    stream_io,
    types::{HttpRequest, HttpResponse},
};

/// 类型别名：标准化 HTTP 请求结构
pub type Request = HttpRequest;
/// 类型别名：标准化 HTTP 响应结构
pub type Response = HttpResponse;

/// HTTP 响应构建器
///
/// 职责：
/// 构造符合宿主要求的 `HttpResponse` 对象，支持 JSON 数据与文件流资源。
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// 构造 JSON 响应 (200 OK)
    ///
    /// 行为：
    /// 1. 将数据序列化为 JSON 字节数组。
    /// 2. 申请宿主内存缓冲区 (Memory Buffer)。
    /// 3. 返回包含缓冲区句柄的响应对象。
    ///
    /// 异常处理：
    /// 若序列化失败，默认返回空 JSON 数组 `[]`，防止插件崩溃。
    pub fn json<T: serde::Serialize>(data: &T) -> Response {
        let json_bytes = serde_json::to_vec(data).unwrap_or_else(|_| b"[]".to_vec());

        // 申请内存资源，生命周期由宿主管理
        let buffer = stream_io::create_memory_buffer(&json_bytes);
        HttpResponse {
            status: 200,
            body: Some(buffer),
        }
    }

    /// 构造文件流响应 (200 OK)
    ///
    /// 行为：
    /// 1. 请求宿主打开指定 UUID 的视频文件资源。
    /// 2. 若成功，返回文件资源句柄；若失败，返回 404。
    ///
    /// 注意：
    /// 插件仅负责传递文件句柄 (Handle)，具体的流式传输 (Streaming)、
    /// Range 请求处理 (Seeking) 均由宿主 (vtx-core) 在 Web 层统一处理。
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

    /// 构造纯状态码响应 (无 Body)
    pub fn status(code: u16) -> Response {
        HttpResponse {
            status: code,
            body: None,
        }
    }

    /// 构造 404 Not Found 响应
    pub fn not_found() -> Response {
        Self::status(404)
    }
}
