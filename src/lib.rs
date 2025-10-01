use std::sync::Arc;

use poem::{
    EndpointExt, Route, Server,
    http::Method,
    listener::TcpListener,
    middleware::{Cors, Tracing},
};
use poem_openapi::OpenApiService;
use tokio::sync::RwLock;

use crate::{
    api::auth::AppApi,
    domain::error::app_error::{AppError, AppResult},
    services::supabase_auth_service::SupabaseAuthService,
    state::AppState,
    utils::config::AppConfig,
};

pub mod api;
pub mod domain;
pub mod frontend;
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
            config.supabase_url.clone(),
            config.supabase_anon_key.clone(),
            config.supabase_service_role_key.clone(),
        )));
        let state = AppState::new(auth_service, config.supabase_jwt_secret.clone());
        App { config, state }
    }

    pub async fn run(&self) -> AppResult<()> {
        // OpenAPI service - use HTTP since Caddy handles TLS
        let api_service = OpenApiService::new(AppApi, "BreezeEHR API", "1.0")
            .server(format!("http://{}", self.config.app_address));
        let ui = api_service.swagger_ui();

        // CORS - allow Caddy's domains
        let cors = Cors::new()
            .allow_origin("https://localhost:8443")
            .allow_origin("https://127.0.0.1:8443")
            .allow_origin("https://breezeehr.ddrcode.me")
            .allow_origin("https://www.breezeehr.ddrcode.me")
            .allow_methods(vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers(vec!["Authorization", "Content-Type"])
            .expose_headers(vec!["Content-Length"])
            .max_age(3600);

        let app = Route::new()
            .nest("/api", api_service)
            .nest("/docs", ui)
            .nest("/", frontend::build_frontend_routes())
            .with(Tracing)
            .with(cors)
            .data(self.state.clone());

        // Simple HTTP listener - no TLS!
        let listener = TcpListener::bind(&self.config.app_address);

        println!(
            "Starting HTTP server on {} (TLS handled by Caddy)",
            self.config.app_address
        );

        Server::new(listener)
            .run(app)
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
