//! Host-side event bus helpers.

use crate::bindings::vtx::api::event_bus;
use crate::error::{VtxError, VtxResult};

/// 发布一个事件（payload 为已序列化的 JSON 字符串）。
pub fn publish_raw(topic: &str, payload_json: &str) -> VtxResult<()> {
    event_bus::publish_event(topic, payload_json).map_err(|e| {
        // 宿主侧会先做 JSON 校验，常见错误是 Invalid event payload
        if e.to_lowercase().contains("invalid event payload") {
            VtxError::SerializationError(e)
        } else {
            VtxError::from_host_message(e)
        }
    })
}

/// 发布一个事件（payload 自动序列化为 JSON）。
pub fn publish_json<T: serde::Serialize>(topic: &str, payload: &T) -> VtxResult<()> {
    let payload_json =
        serde_json::to_string(payload).map_err(|e| VtxError::SerializationError(e.to_string()))?;
    publish_raw(topic, &payload_json)
}
