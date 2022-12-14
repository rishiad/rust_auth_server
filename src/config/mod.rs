pub mod crypto;

use std::{time::Duration, sync::Arc};
use eyre::WrapErr;
use color_eyre::Result;
use serde::Deserialize;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing_subscriber::EnvFilter;
use tracing::{info, instrument};

use self::crypto::CryptoService;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String
}

impl Config {

    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading config...");
        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;
        c.try_into()
            .context("loading config from env")
    }

    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating DB connection pool...");

        Ok(PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.database_url)
            .await
            .context("Connecting to database pool...")?)
    }

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService { key: Arc::new(self.secret_key.clone()), jwt_secret: Arc::new(self.jwt_secret.clone()) }
    }
}