use axum::Router;
use axum::routing::get;
use axum::serve::Serve;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use crate::routes::health_check::health_check;

pub mod configuration;
mod routes;

pub fn run_server(
    tcp_listener:  tokio::net::TcpListener,
    connection_pool : PgPool
) -> Serve<Router, Router> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(connection_pool);
    axum::serve(tcp_listener, app)
}