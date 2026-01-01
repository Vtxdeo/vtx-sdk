use crate::bindings::vtx::api::stream_io;
use crate::bindings::vtx::api::stream_io::Buffer;
use crate::error::{VtxError, VtxResult};
use serde::de::DeserializeOwned;

pub type StreamBuffer = Buffer;

/// 通过宿主的 UUID 打开文件并返回 `Buffer` 资源句柄。
pub fn open_file(uuid: &str) -> VtxResult<Buffer> {
    stream_io::open_file(uuid).map_err(VtxError::from_host_message)
}

/// 创建一个内存 Buffer（通常用于构造 `HttpResponse.body`）。
pub fn memory_buffer(data: impl AsRef<[u8]>) -> Buffer {
    stream_io::create_memory_buffer(data.as_ref())
}

/// `Buffer` 资源的便捷扩展方法。
pub trait BufferExt {
    /// 读取整个 Buffer。
    ///
    /// - 对于 file/memory：使用 `size()` 做分块读取。
    /// - 对于 pipe：会持续读取直到 EOF（返回空数组）或到达 `max_total_bytes`。
    fn read_all(&self) -> Vec<u8>;

    /// 按 UTF-8 读取整个 Buffer。
    fn read_to_string(&self) -> VtxResult<String>;

    /// 将 Buffer 中的 JSON 反序列化为目标类型。
    fn read_json<T: DeserializeOwned>(&self) -> VtxResult<T>;

    /// 向 Buffer 追加写入（对文件：append；对 pipe：写入 stdin；对 memory：append）。
    fn write_all(&self, data: impl AsRef<[u8]>) -> u64;
}

impl BufferExt for Buffer {
    fn read_all(&self) -> Vec<u8> {
        const CHUNK: u64 = 64 * 1024;
        const MAX_TOTAL: usize = 64 * 1024 * 1024;

        let mut out = Vec::new();

        let total = self.size();
        if total > 0 {
            let mut offset = 0u64;
            while offset < total && out.len() < MAX_TOTAL {
                let to_read = std::cmp::min(CHUNK, total - offset);
                let chunk = self.read(offset, to_read);
                if chunk.is_empty() {
                    break;
                }
                out.extend_from_slice(&chunk);
                offset += chunk.len() as u64;
            }
            return out;
        }

        // Pipe 模式：不知道 size，读到空为止（EOF）。
        while out.len() < MAX_TOTAL {
            let chunk = self.read(0, CHUNK);
            if chunk.is_empty() {
                break;
            }
            out.extend_from_slice(&chunk);
        }

        out
    }

    fn read_to_string(&self) -> VtxResult<String> {
        let bytes = self.read_all();
        String::from_utf8(bytes).map_err(|e| VtxError::SerializationError(e.to_string()))
    }

    fn read_json<T: DeserializeOwned>(&self) -> VtxResult<T> {
        let s = self.read_to_string()?;
        serde_json::from_str(&s).map_err(|e| VtxError::SerializationError(e.to_string()))
    }

    fn write_all(&self, data: impl AsRef<[u8]>) -> u64 {
        self.write(data.as_ref())
    }
}
