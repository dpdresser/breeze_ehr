use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::interfaces::auth_service::AuthService;

type AuthServiceType = Arc<RwLock<dyn AuthService + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthServiceType,
    pub email_confirm_redirect: Option<String>,
}

impl AppState {
    pub fn new(auth_service: AuthServiceType, email_confirm_redirect: Option<String>) -> Self {
        AppState {
            auth_service,
            email_confirm_redirect,
        }
    }
}
