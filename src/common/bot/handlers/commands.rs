use crate::{
    bot::{
        dialogue::update_dialogue,
        schema::{Command, MyDialogue, State},
        text_messages::TextMessage,
    },
    models::{course::Course, participant::Participant, signup::SignupStatus},
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
    update_dialogue(State::ReceiveGivenName(true), bot, dialogue, &pool).await?;
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
    match Course::today(&pool).await? {
        Some(course) => {
            let participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
            match participant.signup(&pool, course.id).await? {
                Some(signup) => match signup.status {
                    SignupStatus::SignedUp => {
                        bot.send_message(msg.chat.id, "Du bist bereits angemeldet. Um dich abzumelden, musst du beim UniSport anrufen.").await?;
                    }
                    _ => {
                        update_dialogue(
                            State::ReceiveSignupResponse(course.id),
                            bot,
                            dialogue,
                            &pool,
                        )
                        .await
                        .unwrap();
                    }
                },
                None => {
                    update_dialogue(
                        State::ReceiveSignupResponse(course.id),
                        bot,
                        dialogue,
                        &pool,
                    )
                    .await
                    .unwrap();
                    participant
                        .set_signup_status(&pool, course.id, SignupStatus::Notified)
                        .await?;
                }
            }
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Für heute habe ich leider keine Kurse gefunden.",
            )
            .reply_markup(KeyboardRemove::default())
            .await?;
            dialogue.reset().await.unwrap();
        }
    };

    Ok(())
}

pub async fn edit_given_name(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_given_name by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceiveGivenName(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_last_name(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_last_name by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceiveLastName(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_gender(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_gender by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceiveGender(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_street(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_street by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceiveStreet(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_city(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_city by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceiveCity(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_phone(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_phone by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceivePhone(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_email(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_email by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceiveEmail(false, None), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_status(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<()> {
    log::info!("edit_status by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceiveStatus(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_status_related_info(
    bot: Bot,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("edit_status_related_info by chat {}", dialogue.chat_id());
    update_dialogue(State::ReceiveStatusRelatedInfo(false), bot, dialogue, &pool).await?;
    Ok(())
}
