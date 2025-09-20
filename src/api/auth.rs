use poem::web::Data;
use poem_openapi::{OpenApi, payload::Json};

use crate::{
    domain::error::http_response::AppHttpResponse,
    routes::auth::{
        delete_user::{DeleteUserRequest, delete_user_handler},
        guard::AuthenticatedUser,
        retrieve_user_id::{RetrieveUserIdRequest, retrieve_user_id_handler},
        signin::{SigninRequest, signin_handler},
        signout::signout_handler,
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

    #[oai(path = "/auth/delete_user", method = "delete")]
    #[tracing::instrument(name = "delete_user", skip_all, fields(req_id=%ctx.request_id))]
    async fn delete_user(
        &self,
        ctx: RequestContext,
        _auth: AuthenticatedUser,
        state: Data<&AppState>,
        payload: Json<DeleteUserRequest>,
    ) -> AppHttpResponse {
        match delete_user_handler(state, payload).await {
            Ok(()) => AppHttpResponse::Ok(Json(
                serde_json::json!({ "message": "User deleted successfully" }),
            )),
            Err(e) => AppHttpResponse::from_app_error(e, &ctx.request_id),
        }
    }

    #[oai(path = "/auth/retrieve_user_id", method = "post")]
    #[tracing::instrument(name = "retrieve_user_id", skip_all, fields(req_id=%ctx.request_id))]
    async fn retrieve_user_id(
        &self,
        ctx: RequestContext,
        _auth: AuthenticatedUser,
        state: Data<&AppState>,
        payload: Json<RetrieveUserIdRequest>,
    ) -> AppHttpResponse {
        match retrieve_user_id_handler(state, payload).await {
            Ok(response) => {
                AppHttpResponse::Ok(Json(serde_json::json!({ "user_id": response.user_id })))
            }
            Err(e) => AppHttpResponse::from_app_error(e, &ctx.request_id),
        }
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

    #[oai(path = "/auth/signout", method = "post")]
    #[tracing::instrument(name = "signout", skip_all, fields(req_id=%ctx.request_id))]
    async fn signout(
        &self,
        ctx: RequestContext,
        auth: AuthenticatedUser,
        state: Data<&AppState>,
    ) -> AppHttpResponse {
        match signout_handler(state, auth).await {
            Ok(()) => AppHttpResponse::Ok(Json(
                serde_json::json!({ "message": "User signed out successfully" }),
            )),
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
