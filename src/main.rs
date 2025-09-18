use sovaehr::{
    App,
    utils::{config::AppConfig, tracing::init_tracing},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();
    init_tracing(&config.log_level);

    let app = App::new(config);
    app.run().await?;

    Ok(())
}
