use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use template::configuration::get_configuration;
use template::run_server;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "template".into(),
        std::io::stdout
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("failed to set subscriber");
    
    
    let connection_pool: PgPool =
        PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let address = (
        configuration.application.host,
        configuration.application.port,
    );
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to bind to port:  {}",
                configuration.application.port
            )
        });

    run_server(listener, connection_pool.clone()).await.unwrap();
}
