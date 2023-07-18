extern crate pretty_env_logger;

use color_eyre::Result;
use common::bot;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    match dotenv::dotenv() {
        Ok(_) => log::info!("initialized environment from .env file"),
        Err(err) => log::warn!("did not initialize dotenv: {err}"),
    }
    pretty_env_logger::init();

    log::info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    let redis_url = env::var("REDIS_URL")?;
    log::info!("starting bot");
    bot::start(pool, redis_url).await?;

    Ok(())
}
