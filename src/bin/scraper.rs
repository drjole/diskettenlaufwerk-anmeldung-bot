extern crate pretty_env_logger;

use color_eyre::{eyre::eyre, Result};
use common::{
    bot::{
        keyboards::signup_keyboard,
        schema::{MyStorage, State},
    },
    models::{course::Course, participant::Participant, signup::SignupStatus},
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use teloxide::{
    dispatching::dialogue::{serializer::Bincode, RedisStorage, Storage},
    prelude::*,
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    match dotenv::dotenv() {
        Ok(path) => log::info!(
            "initialized environment from this file: {}",
            path.to_str()
                .ok_or_else(|| eyre!("could not convert path to dotenv file to str"))?
        ),
        Err(err) => log::warn!("did not initialize dotenv: {err}"),
    }
    pretty_env_logger::init_timed();

    log::info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    log::info!("fetching new courses");
    Course::fetch(&pool).await?;

    let course_today = match Course::today(&pool).await? {
        Some(c) => c,
        None => {
            log::info!("no course found for today");
            return Ok(());
        }
    };
    let uninformed_participants = Participant::uninformed(&course_today, &pool).await?;

    let bot = Bot::from_env();
    let redis_url = env::var("REDIS_URL")?;
    let storage: MyStorage = RedisStorage::open(redis_url, Bincode).await?.erase();
    for participant in &uninformed_participants {
        let state = storage
            .clone()
            .get_dialogue(ChatId(participant.id))
            .await
            .unwrap()
            .unwrap();
        // Only inform participants that are not currently entering data or doing something else.
        if !matches!(state, State::Default) {
            continue;
        }

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

        storage
            .clone()
            .update_dialogue(ChatId(participant.id), State::ReceiveSignupResponse)
            .await
            .unwrap();

        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
