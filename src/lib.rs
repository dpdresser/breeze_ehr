use std::sync::Arc;

use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Tracing};
use poem_openapi::OpenApiService;
use tokio::sync::RwLock;

use crate::{
    api::AppApi, domain::error::app_error::AppResult,
    services::supabase_auth_service::SupabaseAuthService, state::AppState,
    utils::config::AppConfig,
};

pub mod api;
pub mod domain;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;

pub struct App {
    pub config: AppConfig,
    pub state: AppState,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let auth_service = Arc::new(RwLock::new(SupabaseAuthService::new(
            &config.supabase_url,
            &config.supabase_key,
        )));
        let state = AppState::new(auth_service);
        App { config, state }
    }

    pub async fn run(&self) -> AppResult<()> {
        let api_service = OpenApiService::new(AppApi, "SovaEHR API", "1.0")
            .server(format!("http://{}", self.config.app_address));
        let ui = api_service.swagger_ui();

        let app = Route::new()
            .nest("/api", api_service)
            .nest("/docs", ui)
            .with(Tracing)
            .data(self.state.clone());

        let listener = TcpListener::bind(&self.config.app_address);
        Server::new(listener)
            .run(app)
            .await
            .expect("Failed to start server");

        Ok(())
    }
}
