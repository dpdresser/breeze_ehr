use std::sync::Arc;

use poem::{
    EndpointExt, Route, Server,
    http::Method,
    listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener},
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
        let state = AppState::new(
            auth_service,
            config.email_confirm_redirect.clone(),
            config.supabase_jwt_secret.clone(),
        );
        App { config, state }
    }

    pub async fn run(&self) -> AppResult<()> {
        // OpenAPI service
        let api_service = OpenApiService::new(AppApi, "SovaEHR API", "1.0")
            .server(format!("https://{}", self.config.app_address));
        let ui = api_service.swagger_ui();

        // CORS
        let cors = Cors::new()
            .allow_origin("https://localhost:3000")
            .allow_origin("https://127.0.0.1:3000")
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

        // TLS config
        let cert_data = std::fs::read(&self.config.tls_cert_path).map_err(AppError::internal)?;
        let key_data = std::fs::read(&self.config.tls_key_path).map_err(AppError::internal)?;
        let cert = RustlsCertificate::new().key(key_data).cert(cert_data);
        let rustls_config = RustlsConfig::new().fallback(cert);

        let listener = TcpListener::bind(&self.config.app_address).rustls(rustls_config);
        Server::new(listener)
            .run(app)
            .await
            .map_err(AppError::internal)?;

        Ok(())
    }
}
