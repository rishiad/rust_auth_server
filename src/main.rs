#[macro_use]
extern crate validator_derive;

mod config;
mod models;
mod handlers;
mod db;
mod errors;

use color_eyre::Result;
use tracing::{info, instrument};
use crate::{config::Config, handlers::app_config};
use actix_web::{App, HttpServer, middleware::Logger, web::Data};

#[actix_rt::main]
#[instrument]
async fn main() -> Result<()> {
    let config: Config = Config::from_env()
                    .expect("Server Config");
    let pool = config.db_pool().await
        .expect("Database Config");
    let cryptoService = config.crypto_service();
    info!("Starting Server...");
    HttpServer::new( move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(cryptoService.clone()))
            .configure(app_config)
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
    Ok(())
}
