use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use dotenv::dotenv;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub host: String,
  pub port: i32,
  pub database_url: String
}

impl Config {

  #[instrument]
  pub fn from_env() -> Result<Config> {
    dotenv().ok();

    tracing_subscriber::fmt()
      .with_env_filter(EnvFilter::from_default_env())
      .init();

      info!("Loading configuration");

    let mut c = config::Config::new();

    c.merge(config::Environment::default())?;

    c.try_into()
      .context("loading configuration from environment")
  }

  #[instrument(skip(self))]
  pub async fn db_pool(&self) -> Result<PgPool> {
      info!("Creating database connection pool.");
      PgPool::builder()
          .connect_timeout(std::time::Duration::from_secs(30))
          .build(&self.database_url)
          .await
          .context("creating database connection pool")
  }
}