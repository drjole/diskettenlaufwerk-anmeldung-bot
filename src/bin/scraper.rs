extern crate encoding;
extern crate pretty_env_logger;

use color_eyre::Result;
use common::{
    bot::keyboards::signup_keyboard,
    models::{course::Course, participant::Participant, signup::SignupStatus},
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use teloxide::prelude::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();

    log::info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    log::info!("fetching new courses");
    Course::fetch(&pool).await?;
    let course_today = match Course::today(&pool).await? {
        Some(c) => c,
        None => return Ok(()),
    };

    let uninformed_participants = Participant::uninformed(&course_today, &pool).await?;

    let bot = Bot::from_env();
    for participant in &uninformed_participants {
        log::info!("informing participant {}", participant.id);
        bot.send_message(
            ChatId(participant.id),
            format!(
                r#"Heute ist Frisbee-Zeit!

{course_today}

Soll ich dich anmelden?"#
            ),
        )
        .reply_markup(signup_keyboard(course_today.id))
        .await?;
        participant
            .set_signup_status(&pool, course_today.id, SignupStatus::Notified)
            .await?;
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
