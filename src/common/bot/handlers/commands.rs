use crate::{
    bot::{
        dialogue::update_dialogue,
        schema::{MyDialogue, State},
    },
    models::participant::Participant,
};
use anyhow::Result;
use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

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

pub async fn show_data(bot: Bot, msg: Message, pool: Pool<Postgres>) -> Result<()> {
    log::info!("show_data by chat {}", msg.chat.id);
    let participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    bot.send_message(
        msg.chat.id,
        format!(
            r#"Ich habe folgende Informationen über dich gespeichert. Nutze die angezeigten Befehle, um deine Daten zu ändern.

{participant}"#
        ),
    )
    .await?;
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
    update_dialogue(State::ReceiveEmail(false), bot, dialogue, &pool).await?;
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
