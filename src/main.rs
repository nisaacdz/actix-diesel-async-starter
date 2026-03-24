use infra::settings::Settings;
use tracing::info;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let run_mode = std::env::var("RUN_MODE")
        .ok()
        .filter(|m| ["production", "development", "test"].contains(&m.as_str()))
        .unwrap_or("development".to_string());

    let settings: Settings = config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()
        .expect("Failed to build configuration")
        .try_deserialize()
        .expect("Failed to deserialize configuration");

    info!(
        "Starting server at http://{}:{} in {run_mode:?}",
        settings.server.host, settings.server.port
    );

    api::run(settings).await
}
