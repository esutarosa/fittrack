pub mod auth;
pub mod error;
pub mod http;
pub mod storage;
pub mod training;

use anyhow::Result;
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

pub async fn run() -> Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = storage::ServerConfig::from_env()?;
    let state = storage::AppState::new(&config).await?;
    storage::init_schema(&state.pool).await?;

    let listener = TcpListener::bind(config.bind_addr).await?;
    let app = http::router(state);

    axum::serve(listener, app).await?;
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = fmt::layer().with_target(false);
    tracing_subscriber::registry().with(filter).with(subscriber).init();
}
