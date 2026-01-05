//! Plugin-side HTTP client helpers.

use crate::bindings::vtx::api::{
    http_client,
    types::{HttpClientRequest, HttpClientResponse},
};
use crate::error::{VtxError, VtxResult};

pub type Request = HttpClientRequest;
pub type Response = HttpClientResponse;

pub fn request(req: Request) -> VtxResult<Response> {
    http_client::request(req).map_err(VtxError::from_host_message)
}
