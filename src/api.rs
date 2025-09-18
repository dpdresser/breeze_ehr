use poem_openapi::OpenApi;

use crate::{domain::error::http_response::AppHttpResponse, utils::tracing::RequestContext};

#[derive(Debug)]
pub struct AppApi;

#[OpenApi]
impl AppApi {
    #[oai(path = "/health", method = "get")]
    #[tracing::instrument(name = "health_check", skip_all, fields(req_id=%ctx.request_id))]
    async fn health_check(&self, ctx: RequestContext) -> AppHttpResponse {
        AppHttpResponse::Ok(poem_openapi::payload::Json(
            serde_json::json!({"status": "ok"}),
        ))
    }
}
