use infra::db;
use infra::settings::Settings;

#[tokio::main]
async fn main() {
    let settings = {
        dotenvy::from_filename(".env").ok();

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

        settings
    };

    let pool = db::init_pool(&settings.database).expect("Failed to connect to database");

    println!("Running migrations...");

    match migrator::run_migrations(&pool).await {
        Ok(_) => println!("✓ Migrations completed successfully"),
        Err(e) => {
            eprintln!("✗ Migration error: {}", e);
            std::process::exit(1);
        }
    }
}
