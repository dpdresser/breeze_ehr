use poem::web::Data;
use poem_openapi::{OpenApi, payload::Json};

use crate::{
    domain::error::http_response::AppHttpResponse,
    routes::{
        signin::{SigninRequest, signin_handler},
        signup::{SignupRequest, signup_handler},
    },
    state::AppState,
    utils::tracing::RequestContext,
};

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

    #[oai(path = "/auth/signin", method = "post")]
    #[tracing::instrument(name = "signin", skip_all, fields(req_id=%ctx.request_id))]
    async fn signin(
        &self,
        ctx: RequestContext,
        state: Data<&AppState>,
        payload: Json<SigninRequest>,
    ) -> AppHttpResponse {
        match signin_handler(state, payload).await {
            Ok(response) => {
                AppHttpResponse::Ok(Json(serde_json::json!({ "token": response.token })))
            }
            Err(e) => AppHttpResponse::from_app_error(e, &ctx.request_id),
        }
    }

    #[oai(path = "/auth/signup", method = "post")]
    #[tracing::instrument(name = "signup", skip_all, fields(req_id=%ctx.request_id))]
    async fn signup(
        &self,
        ctx: RequestContext,
        state: Data<&AppState>,
        payload: Json<SignupRequest>,
    ) -> AppHttpResponse {
        match signup_handler(state, payload).await {
            Ok(response) => {
                AppHttpResponse::Created(Json(serde_json::json!({ "message": response.message })))
            }
            Err(e) => AppHttpResponse::from_app_error(e, &ctx.request_id),
        }
    }
}
