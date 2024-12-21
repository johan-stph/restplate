use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use template::configuration::get_configuration;
use template::run_server;

#[tokio::main]
async fn main() {
    let configuration = get_configuration()
        .expect("Failed to read configuration");

    let connection_pool: PgPool = PgPoolOptions::new().connect_lazy_with(
        configuration.database.with_db()
    );
    
    let address = (configuration.application.host, configuration.application.port);
    let listener =  tokio::net::TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to port:  {}", configuration.application.port));
    
    run_server(listener, connection_pool.clone())
        .await
        .unwrap();

}