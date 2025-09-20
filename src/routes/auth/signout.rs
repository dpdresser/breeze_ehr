use poem::web::Data;
use poem_openapi::{Object, payload::Json};

use crate::{domain::error::app_error::AppResult, state::AppState};

#[derive(Object, Debug)]
pub struct SignoutRequest {
    pub token: String,
}

pub async fn signout_handler(
    state: Data<&AppState>,
    payload: Json<SignoutRequest>,
) -> AppResult<()> {
    state
        .auth_service
        .read()
        .await
        .signout(&payload.token)
        .await?;

    Ok(())
}
