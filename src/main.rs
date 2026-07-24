use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

use crate::config::Config;

mod app;
mod config;
mod db;
mod dto;
mod error;
mod handlers;
mod helper;
mod models;
mod repository;
mod router;
mod services;
mod extractors;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    let config: Config = Config::load();

    registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .init();

    app::run(config).await
}
