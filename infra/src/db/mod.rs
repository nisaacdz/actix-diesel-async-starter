use crate::settings::DatabaseSettings;
use diesel::result::ConnectionError;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{
        AsyncDieselConnectionManager, ManagerConfig,
        deadpool::{BuildError, Pool},
    },
};
use rustls::{ClientConfig, RootCertStore};
use std::str::FromStr;
use tokio_postgres::{NoTls, config::SslMode};
use tokio_postgres_rustls::MakeRustlsConnect;

pub type DbPool = Pool<AsyncPgConnection>;

fn make_tls() -> MakeRustlsConnect {
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let rustls_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    MakeRustlsConnect::new(rustls_config)
}

async fn establish_connection(
    url: &str,
    tls: MakeRustlsConnect,
) -> Result<AsyncPgConnection, ConnectionError> {
    let mut config = tokio_postgres::Config::from_str(url).map_err(|e| {
        ConnectionError::BadConnection(format!("Failed to parse database URL: {}", e))
    })?;

    // Relax channel binding requirement to avoid authentication errors with some providers (e.g. Neon)
    config.channel_binding(tokio_postgres::config::ChannelBinding::Prefer);

    let ssl_mode = config.get_ssl_mode();
    tracing::debug!("Connecting to database with SSL mode: {:?}", ssl_mode);

    let client = match ssl_mode {
        SslMode::Disable => {
            tracing::debug!("Using NoTls connection");
            let (client, connection) = config.connect(NoTls).await.map_err(|e| {
                ConnectionError::BadConnection(format!("Database connection failed (NoTls): {}", e))
            })?;
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    tracing::error!("Database connection error (NoTls): {}", e);
                }
            });
            client
        }
        _ => {
            tracing::debug!("Using TLS connection");
            let (client, connection) = config
                .connect(tls)
                .await
                .map_err(|e| ConnectionError::BadConnection(format!("Database TLS connection failed: {}. This may be a TLS version mismatch or certificate issue.", e)))?;
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    tracing::error!("Database connection error (TLS): {}", e);
                }
            });
            client
        }
    };

    AsyncPgConnection::try_from(client).await.map_err(|e| {
        ConnectionError::BadConnection(format!(
            "Failed to create async connection from client: {}",
            e
        ))
    })
}

/// Initialize database connection pool
pub fn init_pool(db_config: &DatabaseSettings) -> Result<DbPool, BuildError> {
    let tls = make_tls();

    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(move |url| {
        let tls = tls.clone();
        Box::pin(establish_connection(url, tls))
    });

    let manager =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(&db_config.url, config);

    Pool::builder(manager)
        .max_size(db_config.max_connections)
        .build()
}
