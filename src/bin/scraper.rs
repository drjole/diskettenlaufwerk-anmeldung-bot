use color_eyre::Result;
use common::{
    bot::keyboards::signup_keyboard,
    models::{course::Course, participant::Participant, signup::SignupStatus},
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();
    dotenv()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    Course::fetch(&pool).await?;
    let course_today = Course::today(&pool).await?;

    let uninformed_participants = Participant::uninformed(&course_today, &pool).await?;

    let bot = Bot::from_env();
    for participant in &uninformed_participants {
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

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
