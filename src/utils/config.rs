#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_address: String,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let app_host = std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let app_port = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
        let app_address = format!("{app_host}:{app_port}");

        let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        AppConfig {
            app_address,
            log_level,
        }
    }

    pub fn for_tests() -> Self {
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let app_address = format!("127.0.0.1:{port}");

        let log_level = "info".to_string();

        AppConfig {
            app_address,
            log_level,
        }
    }
}
