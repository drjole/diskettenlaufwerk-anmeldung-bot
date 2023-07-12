use color_eyre::Result;
use common::models::{course::Course, participant::Participant};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();
    dotenv()?;

    let courses = Course::download().await?;
    if courses.is_empty() {
        return Ok(());
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    let bot = Bot::from_env();
    let participants = Participant::all(&pool).await?;
    for participant in &participants {}

    Ok(())
}
