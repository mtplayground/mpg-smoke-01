mod app;
mod error;
mod models;
mod routes;
mod state;

use std::env;
use std::net::{Ipv4Addr, SocketAddr};

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::app::build_router;
use crate::error::AppError;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init_tracing();

    let database_url = env::var("DATABASE_URL").map_err(|_| AppError::missing_env("DATABASE_URL"))?;
    let port = read_port()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|error| AppError::database_connection("connect DATABASE_URL", error))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|error| AppError::database_connection("run embedded migrations", error.into()))?;

    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)))
        .await
        .map_err(AppError::bind)?;

    let router = build_router(AppState { pool });

    tracing::info!("listening on 0.0.0.0:{port}");
    axum::serve(listener, router)
        .await
        .map_err(AppError::server)?;

    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn read_port() -> Result<u16, AppError> {
    match env::var("PORT") {
        Ok(raw) => raw.parse::<u16>().map_err(|_| AppError::invalid_env("PORT", raw)),
        Err(_) => Ok(8080),
    }
}
