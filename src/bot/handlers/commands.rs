use crate::{
    bot::{
        dialogue_utils,
        schema::{Command, MyDialogue, State},
        text_messages::TextMessage,
    },
    models::{course::Course, participant::Participant, signup::Status},
};
use color_eyre::Result;
use sqlx::{Pool, Postgres};
use teloxide::{prelude::*, types::KeyboardRemove, utils::command::BotCommands};

pub async fn help(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    log::info!("help by chat {}", msg.chat.id);
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .reply_markup(KeyboardRemove::default())
        .await?;
    dialogue.reset().await.unwrap();
    Ok(())
}

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    log::info!("start by chat {}", msg.chat.id);
    bot.send_message(msg.chat.id, TextMessage::Start.to_string())
        .reply_markup(KeyboardRemove::default())
        .await?;
    dialogue.reset().await.unwrap();
    Ok(())
}

pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    log::info!("cancel by chat {}", msg.chat.id);
    bot.send_message(msg.chat.id, TextMessage::Cancel.to_string())
        .reply_markup(KeyboardRemove::default())
        .await?;
    dialogue.reset().await.unwrap();
    Ok(())
}

pub async fn enter_data(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("enter_data by chat {}", msg.chat.id);
    if (Participant::find_by_id(&pool, msg.chat.id.0).await).is_err() {
        let participant = Participant {
            id: msg.chat.id.0,
            ..Default::default()
        };
        participant.create(&pool).await?;
    }
    dialogue_utils::update(State::ReceiveGivenName(true), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn show_data(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("show_data by chat {}", msg.chat.id);
    let participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    bot.send_message(msg.chat.id, TextMessage::ShowData(participant).to_string())
        .parse_mode(teloxide::types::ParseMode::Html)
        .reply_markup(KeyboardRemove::default())
        .await?;
    dialogue.reset().await.unwrap();
    Ok(())
}

pub async fn signup(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("signup by chat {}", msg.chat.id);
    if let Some(course) = Course::today(&pool).await? {
        let participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
        if let Some(signup) = participant.signup(&pool, course.id).await? {
            match signup.status {
                Status::SignedUp => {
                    bot.send_message(msg.chat.id, "Du bist bereits angemeldet. Um dich abzumelden, musst du beim UniSport anrufen.").await?;
                }
                _ => {
                    dialogue_utils::update(
                        State::ReceiveSignupResponse(course.id),
                        bot,
                        dialogue,
                        &pool,
                    )
                    .await
                    .unwrap();
                }
            }
        } else {
            dialogue_utils::update(
                State::ReceiveSignupResponse(course.id),
                bot,
                dialogue,
                &pool,
            )
            .await
            .unwrap();
            participant
                .set_signup_status(&pool, course.id, Status::Notified)
                .await?;
        }
    } else {
        bot.send_message(
            msg.chat.id,
            "Für heute habe ich leider keine Kurse gefunden.",
        )
        .reply_markup(KeyboardRemove::default())
        .await?;
        dialogue.reset().await.unwrap();
    };

    Ok(())
}

pub async fn delete(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("delete by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveDeleteConfirmation, bot, dialogue, &pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn edit_given_name(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_given_name by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveGivenName(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_last_name(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_last_name by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveLastName(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_gender(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_gender by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveGender(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_street(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_street by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveStreet(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_city(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_city by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveCity(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_phone(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_phone by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceivePhone(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_email(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_email by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveEmail(false, None), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_status(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_status by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveStatus(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_status_info(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_status_info by chat {}", dialogue.chat_id());
    dialogue_utils::update(State::ReceiveStatusInfo(false), bot, dialogue, &pool).await?;
    Ok(())
}
