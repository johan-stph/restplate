use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use template::configuration::{DatabaseSettings, get_configuration};
use uuid::Uuid;
use template::run_server;

pub struct MockApp {
    pub address: String,
    pub db_pool: PgPool,
    pub database: DatabaseSettings,
}

pub async fn spawn_app_local() -> MockApp {

    let local_host = "127.0.0.1";
    let ip_address = format!("{}:0", local_host);
    let tcp_listener = tokio::net::TcpListener::bind(ip_address)
        .await
        .expect("failed to bind port");
    let port = tcp_listener
        .local_addr()
        .expect("failed to create ip/port")
        .port();
    let address = format!("http://{}:{}", local_host, port);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_pool: PgPool = configure_database(&configuration.database).await;
    let cloned = db_pool.clone();
    tokio::spawn(async move { run_server(tcp_listener, cloned).await.unwrap() });
    MockApp {
        address,
        db_pool,
        database: configuration.database,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to establish a connection to Postgres Instance.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    let connection_pool: PgPool = PgPoolOptions::new()
        .connect_with(config.with_db())
        .await
        .expect("Failed to establish a connection to postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate");
    connection_pool
}

async fn teardown_database(config: &DatabaseSettings) {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres for teardown.");
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to drop test database.");
}

pub async fn close_and_delete_db(mock_app: MockApp) {
    mock_app.db_pool.close().await;
    teardown_database(&mock_app.database).await;
}
