use std::sync::Arc;

use secrecy::SecretString;
use tokio::sync::RwLock;

use crate::domain::interfaces::auth_service::AuthService;

type AuthServiceType = Arc<RwLock<dyn AuthService + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthServiceType,
    pub email_confirm_redirect: Option<String>,
    pub supabase_jwt_secret: SecretString,
}

impl AppState {
    pub fn new(
        auth_service: AuthServiceType,
        email_confirm_redirect: Option<String>,
        supabase_jwt_secret: SecretString,
    ) -> Self {
        AppState {
            auth_service,
            email_confirm_redirect,
            supabase_jwt_secret,
        }
    }
}
