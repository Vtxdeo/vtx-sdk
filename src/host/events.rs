//! Host-side event types and helpers.

use crate::bindings::vtx::api::events::{EventContext, VtxEvent};
use crate::error::{VtxError, VtxResult};
use serde::de::DeserializeOwned;

pub type PluginEvent = VtxEvent;
pub type PluginEventContext = EventContext;

pub trait VtxEventExt {
    /// 将 `payload`（JSON 字符串）反序列化为目标类型。
    fn payload_json<T: DeserializeOwned>(&self) -> VtxResult<T>;
}

impl VtxEventExt for VtxEvent {
    fn payload_json<T: DeserializeOwned>(&self) -> VtxResult<T> {
        serde_json::from_str(&self.payload).map_err(|e| VtxError::SerializationError(e.to_string()))
    }
}
