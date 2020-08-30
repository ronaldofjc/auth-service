#[macro_use]
extern crate validator_derive;

mod config;
mod handlers;
mod models;

use color_eyre::Result;
use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger};
use tracing::info;
use handlers::app_config;

#[actix_rt::main]
async fn main() -> Result<()> {
    
    let config = Config::from_env()
        .expect("Server configuration");

        info!("Starting server at http://{}:{}/", config.host, config.port);

        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .configure(app_config)
        })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
