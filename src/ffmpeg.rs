// vtx-sdk/src/ffmpeg.rs

use crate::bindings::vtx::api::ffmpeg::{self, TranscodeParams};
use crate::bindings::vtx::api::types::HttpResponse;
use crate::error::{VtxError, VtxResult};

/// FFmpeg 任务构建器
///
/// 用于构建并执行服务端的 FFmpeg 转码任务。
/// 采用 Builder 模式，支持链式调用。
///
/// # Example
///
/// ```rust
/// use vtx_sdk::prelude::*;
///
/// fn handle_video(vid: String) -> VtxResult<Response> {
///     FfmpegTask::new("mini", vid)
///         .arg("-ss 10")
///         .arg("-t 30")
///         .execute()
/// }
/// ```
pub struct FfmpegTask {
    profile: String,
    input_id: String,
    args: Vec<String>,
}

impl FfmpegTask {
    /// 创建一个新的 FFmpeg 任务
    ///
    /// # Parameters
    /// - `profile`: 目标 Profile 名称 (如 "mini", "remux", "thumbnail")
    /// - `input_id`: 输入视频的唯一资源 ID (UUID)
    pub fn new(profile: impl Into<String>, input_id: impl Into<String>) -> Self {
        Self {
            profile: profile.into(),
            input_id: input_id.into(),
            args: Vec::new(),
        }
    }

    /// 添加单个 FFmpeg 参数
    ///
    /// 自动处理参数转义，防止注入风险。
    ///
    /// # Example
    /// `.arg("-ss").arg("10")`
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// 批量添加参数
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    /// 快捷方法：设置输出格式
    /// 等同于 `.arg("-f").arg(format)`
    pub fn format(self, format: &str) -> Self {
        self.arg("-f").arg(format)
    }

    /// 快捷方法：设置时间裁剪
    /// 等同于 `.arg("-ss").arg(start).arg("-t").arg(duration)`
    pub fn seek(self, start: &str, duration: Option<&str>) -> Self {
        let mut s = self.arg("-ss").arg(start);
        if let Some(d) = duration {
            s = s.arg("-t").arg(d);
        }
        s
    }

    /// 执行任务并返回 HTTP 响应
    ///
    /// 该方法会阻塞等待子进程启动，并立即返回包含 stdout 管道流的 HttpResponse。
    /// 数据将以流式传输给客户端，无需等待转码完成。
    pub fn execute(self) -> VtxResult<HttpResponse> {
        let params = TranscodeParams {
            profile: self.profile,
            input_id: self.input_id,
            args: self.args,
        };

        // 调用宿主接口
        match ffmpeg::execute(&params) {
            Ok(buffer_resource) => {
                // 将宿主返回的 Buffer 资源句柄封装进 Response
                Ok(HttpResponse {
                    status: 200,
                    body: Some(buffer_resource),
                })
            }
            Err(e) => {
                // 将宿主错误字符串转换为 SDK 标准错误
                // 通常包含 "Profile not found" 或 "Permission Denied"
                Err(VtxError::Internal(format!(
                    "FFmpeg execution failed: {}",
                    e
                )))
            }
        }
    }
}
