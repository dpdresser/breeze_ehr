use poem::web::Data;

use crate::{
    domain::error::app_error::AppResult, routes::auth::guard::AuthenticatedUser, state::AppState,
};

pub async fn signout_handler(state: Data<&AppState>, auth: AuthenticatedUser) -> AppResult<()> {
    state.auth_service.read().await.signout(&auth.token).await?;

    Ok(())
}
