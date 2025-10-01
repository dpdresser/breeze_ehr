use secrecy::SecretString;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_address: String,
    pub log_level: String,
    pub supabase_url: String,
    pub supabase_anon_key: SecretString,
    pub supabase_service_role_key: SecretString,
    pub supabase_jwt_secret: SecretString,
    pub mailpit_url: String,
    pub email_confirm_redirect: Option<String>,
    pub tls_cert_path: String,
    pub tls_key_path: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let app_host = std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let app_port = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
        let app_address = format!("{app_host}:{app_port}");

        let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        let supabase_url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let supabase_anon_key =
            std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");
        let supabase_service_role_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
            .expect("SUPABASE_SERVICE_ROLE_KEY must be set");
        let supabase_jwt_secret =
            std::env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");

        let mailpit_url =
            std::env::var("MAILPIT_URL").unwrap_or_else(|_| "http://127.0.0.1:54324".to_string());
        let email_confirm_redirect = std::env::var("SIGNUP_CONFIRM_REDIRECT").ok();

        let tls_cert_path = std::env::var("TLS_CERT_PATH").expect("TLS_CERT_PATH must be set");
        let tls_key_path = std::env::var("TLS_KEY_PATH").expect("TLS_KEY_PATH must be set");

        AppConfig {
            app_address,
            log_level,
            supabase_url,
            supabase_anon_key: SecretString::from(supabase_anon_key),
            supabase_service_role_key: SecretString::from(supabase_service_role_key),
            supabase_jwt_secret: SecretString::from(supabase_jwt_secret),
            mailpit_url,
            email_confirm_redirect,
            tls_cert_path,
            tls_key_path,
        }
    }

    pub fn for_tests() -> Self {
        dotenvy::dotenv().ok();

        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let app_address = format!("127.0.0.1:{port}");

        let log_level = "info".to_string();

        let supabase_url = std::env::var("SUPABASE_URL_TEST").expect("SUPABASE_URL must be set");
        let supabase_anon_key =
            std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");
        let supabase_service_role_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
            .expect("SUPABASE_SERVICE_ROLE_KEY must be set");
        let supabase_jwt_secret =
            std::env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");

        let mailpit_url =
            std::env::var("MAILPIT_URL").unwrap_or_else(|_| "http://127.0.0.1:54324".to_string());
        let email_confirm_redirect = std::env::var("SIGNUP_CONFIRM_REDIRECT").ok();

        let tls_cert_path = std::env::var("TLS_CERT_PATH").expect("TLS_CERT_PATH must be set");
        let tls_key_path = std::env::var("TLS_KEY_PATH").expect("TLS_KEY_PATH must be set");

        AppConfig {
            app_address,
            log_level,
            supabase_url,
            supabase_anon_key: SecretString::from(supabase_anon_key),
            supabase_service_role_key: SecretString::from(supabase_service_role_key),
            supabase_jwt_secret: SecretString::from(supabase_jwt_secret),
            mailpit_url,
            email_confirm_redirect,
            tls_cert_path,
            tls_key_path,
        }
    }
}
