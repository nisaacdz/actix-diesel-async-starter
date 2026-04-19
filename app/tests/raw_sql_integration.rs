// use infra::db::{DbPool, init_pool};
// use infra::settings::DatabaseSettings;

// fn init_test_pool() -> DbPool {
//     let database_url = std::env::var("APP__DATABASE__URL")
//         .expect("APP__DATABASE__URL must be set to run DB integration tests");

//     let settings = DatabaseSettings {
//         url: database_url,
//         max_connections: 4,
//     };

//     init_pool(&settings).expect("failed to initialize test DB pool")
// }
