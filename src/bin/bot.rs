extern crate encoding;

use color_eyre::Result;
use common::bot;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();
    dotenv()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    bot::start(pool).await?;

    Ok(())
}
