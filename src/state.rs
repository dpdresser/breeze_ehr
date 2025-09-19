use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::interfaces::auth_service::AuthService;

type AuthServiceType = Arc<RwLock<dyn AuthService + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthServiceType,
}

impl AppState {
    pub fn new(auth_service: AuthServiceType) -> Self {
        AppState { auth_service }
    }
}
