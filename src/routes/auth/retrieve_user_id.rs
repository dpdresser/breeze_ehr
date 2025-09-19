use poem::web::Data;
use poem_openapi::{Object, payload::Json};

use crate::{
    domain::{error::app_error::AppResult, types::email::Email},
    state::AppState,
};

#[derive(Object, Debug)]
pub struct RetrieveUserIdRequest {
    pub email: String,
}

#[derive(Object, Debug)]
pub struct RetrieveUserIdResponse {
    pub user_id: String,
}

pub async fn retrieve_user_id_handler(
    state: Data<&AppState>,
    payload: Json<RetrieveUserIdRequest>,
) -> AppResult<RetrieveUserIdResponse> {
    let email = Email::new(payload.email.clone())?;

    let user_id = state
        .auth_service
        .read()
        .await
        .retrieve_user_id(&email)
        .await?;

    Ok(RetrieveUserIdResponse { user_id })
}
