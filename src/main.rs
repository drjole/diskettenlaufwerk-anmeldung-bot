extern crate pretty_env_logger;

mod bot;
mod models;
mod utils;

use crate::{
    bot::{
        keyboards,
        schema::{MyStorage, State},
        text_messages::TextMessage,
    },
    models::{course::Course, participant::Participant, signup},
};
use color_eyre::{eyre::eyre, Result};
use sqlx::postgres::PgPoolOptions;
use std::env;
use teloxide::{
    dispatching::dialogue::{serializer::Bincode, RedisStorage, Storage},
    prelude::*,
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().map_or_else(
        |_| println!("did not initialize dotenv"),
        |path| {
            println!(
                "initialized dotenv from: {}",
                path.to_str().unwrap_or("unknown")
            );
        },
    );
    pretty_env_logger::init_timed();
    match std::env::args().nth(1).as_deref() {
        Some("bot") | None => run_bot().await,
        Some("scraper") => run_scraper().await,
        _ => Err(eyre!("invalid argument")),
    }
}

async fn run_bot() -> Result<()> {
    log::info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    log::info!("starting bot");
    bot::start(pool, env::var("REDIS_URL")?).await?;

    Ok(())
}

async fn run_scraper() -> Result<()> {
    log::info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    log::info!("fetching new courses");
    Course::fetch(&pool).await?;
    let Some(course_today) = Course::today(&pool).await? else {
        log::info!("no course found for today");
        return Ok(());
    };

    if !course_today.is_signup_available().await {
        log::info!("signup for course for today is not yet available");
        return Ok(());
    }

    let bot = Bot::from_env();
    let storage: MyStorage = RedisStorage::open(&env::var("REDIS_URL")?, Bincode)
        .await?
        .erase();

    log::info!("informing participants");
    for participant in &mut Participant::uninformed(&pool, course_today.id).await? {
        // Only inform participants that are not currently editing their data.
        let dialogue = storage
            .clone()
            .get_dialogue(ChatId(participant.id))
            .await
            .map_err(|e| eyre!(e))?;
        if let Some(state) = dialogue {
            if !matches!(state, State::ReceiveSignupResponse(_) | State::Default) {
                continue;
            }
        } else {
            log::warn!("no dialogue found for participant {}", participant.id);
        }

        if participant.signup_always {
            log::info!(
                "participant {} wants to be signed up always",
                participant.id
            );
            participant
                .set_signup_status(&pool, course_today.id, signup::Status::Notified)
                .await?;
            match signup::perform(participant, course_today.id).await {
                Ok(_) => {
                    participant
                        .set_signup_status(&pool, course_today.id, signup::Status::SignedUp)
                        .await?;
                }
                Err(err) => {
                    log::error!(
                        "failed to sign up participant {} for {}: {}",
                        participant.id,
                        course_today.id,
                        err
                    );
                }
            };
            continue;
        }

        log::info!("informing participant {}", participant.id);
        match bot
            .send_message(
                ChatId(participant.id),
                TextMessage::SignupResponse(course_today.clone()).to_string(),
            )
            .reply_markup(keyboards::signup())
            .await
        {
            Ok(_) => {
                participant
                    .set_signup_status(&pool, course_today.id, signup::Status::Notified)
                    .await?;
                storage
                    .clone()
                    .update_dialogue(
                        ChatId(participant.id),
                        State::ReceiveSignupResponse(course_today.id),
                    )
                    .await
                    .map_err(|e| eyre!(e))?;
                log::info!("successfully informed participant {}", participant.id)
            }
            Err(e) => {
                log::error!("failed to inform participant {}: {}", participant.id, e);
                if e.to_string().contains("bot was blocked by the user") {
                    log::info!(
                        "participant {} blocked the bot, deleting the participant and their dialogue now",
                        participant.id
                    );
                    participant.delete(&pool).await?;
                    storage
                        .clone()
                        .remove_dialogue(ChatId(participant.id))
                        .await
                        .map_err(|e| eyre!(e))?;
                }
                continue;
            }
        };

        log::info!("sleep for 200ms to respect Telegram API rate limiting");
        sleep(Duration::from_millis(200)).await;
    }

    Ok(())
}
