use config::{Config, Environment, File};
use infra::db;
use infra::settings::Settings;

#[tokio::main]
async fn main() {
    let app_settings = {
        dotenvy::from_filename(".env").ok();

        let run_mode = std::env::var("RUN_MODE")
            .ok()
            .filter(|m| ["production", "development", "test"].contains(&m.as_str()))
            .unwrap_or("development".to_string());

        let app_settings: Settings = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
            .expect("Failed to build configuration")
            .try_deserialize()
            .expect("Failed to deserialize configuration");

        app_settings
    };

    let pool = db::init_pool(&app_settings.database).expect("Failed to connect to database");

    println!("Running migrations...");

    match migrator::run_migrations(&pool).await {
        Ok(_) => println!("✓ Migrations completed successfully"),
        Err(e) => {
            eprintln!("✗ Migration error: {}", e);
            std::process::exit(1);
        }
    }
}
