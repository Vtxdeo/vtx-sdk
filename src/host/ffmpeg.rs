//! Host-side FFmpeg helpers.

use crate::bindings::vtx::api::ffmpeg::{self, FfmpegOption, TranscodeProfile};
use crate::bindings::vtx::api::stream_io::Buffer;
use crate::bindings::vtx::api::types::HttpResponse;
use crate::error::{VtxError, VtxResult};

/// FFmpeg task builder for running host-side transcoding.
///
/// # Example
///
/// ```rust
/// use vtx_sdk::prelude::*;
///
/// fn handle_video(vid: String) -> VtxResult<Response> {
///     FfmpegTask::new("mini", vid)
///         .option("ss", "10")
///         .option("t", "30")
///         .execute()
/// }
/// ```
pub struct FfmpegTask {
    profile: String,
    input_id: String,
    options: Vec<FfmpegOption>,
}

impl FfmpegTask {
    /// Create a new FFmpeg task.
    ///
    /// - `profile`: target FFmpeg profile (e.g. "mini", "remux", "thumbnail")
    /// - `input_id`: input resource ID (UUID) or "pipe:0"
    pub fn new(profile: impl Into<String>, input_id: impl Into<String>) -> Self {
        Self {
            profile: profile.into(),
            input_id: input_id.into(),
            options: Vec::new(),
        }
    }

    /// Create a task that uses stdin as input (`input_id = "pipe:0"`).
    pub fn new_pipe(profile: impl Into<String>) -> Self {
        Self::new(profile, "pipe:0")
    }

    /// Add a key/value FFmpeg option (encoded as `-key=value`).
    pub fn option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.push(FfmpegOption {
            key: key.into(),
            value: Some(value.into()),
        });
        self
    }

    /// Add a flag-style FFmpeg option (encoded as `-key`).
    pub fn flag(mut self, key: impl Into<String>) -> Self {
        self.options.push(FfmpegOption {
            key: key.into(),
            value: None,
        });
        self
    }

    /// Add a batch of key/value options.
    pub fn options<I, K, V>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in options {
            self.options.push(FfmpegOption {
                key: key.into(),
                value: Some(value.into()),
            });
        }
        self
    }

    /// Helper: set output format (equivalent to `-f=format`).
    pub fn format(self, format: &str) -> Self {
        self.option("f", format)
    }

    /// Helper: set seek window (equivalent to `-ss` + optional `-t`).
    pub fn seek(self, start: &str, duration: Option<&str>) -> Self {
        let mut s = self.option("ss", start);
        if let Some(d) = duration {
            s = s.option("t", d);
        }
        s
    }

    /// Execute and return the stdout pipe buffer.
    pub fn execute_buffer(self) -> VtxResult<Buffer> {
        let params = TranscodeProfile {
            profile: self.profile,
            input_id: self.input_id,
            options: self.options,
        };

        ffmpeg::execute(&params).map_err(VtxError::from_host_message)
    }

    /// Execute and return an HTTP response (`200` with stdout pipe body).
    pub fn execute(self) -> VtxResult<HttpResponse> {
        let buffer = self.execute_buffer()?;
        Ok(HttpResponse {
            status: 200,
            body: Some(buffer),
        })
    }
}
