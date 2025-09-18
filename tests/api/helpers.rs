use sovaehr::{App, utils::config::AppConfig};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let settings = AppConfig::for_tests();
        let http_client = reqwest::Client::new();

        let app = App::new(settings.clone());

        // Spawn the server
        tokio::spawn(async move { app.run().await.expect("Failed to start test server") });
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        Self {
            address: settings.app_address,
            http_client,
        }
    }

    pub async fn health_check(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("http://{}/api/health", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
