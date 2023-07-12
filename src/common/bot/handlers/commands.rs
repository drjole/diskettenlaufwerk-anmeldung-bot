use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

use crate::{
    bot::{
        schema::{MyDialogue, State},
        utils::update_dialogue,
    },
    models::participant::Participant,
    types::Error,
};

pub async fn enter_data(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    if (Participant::find_by_chat_id(&pool, msg.chat.id.0).await).is_err() {
        let participant = Participant {
            chat_id: msg.chat.id.0,
            ..Default::default()
        };
        participant.insert(&pool).await?;
    }
    update_dialogue(State::ReceiveGivenName(true), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn show_data(bot: Bot, msg: Message, pool: Pool<Postgres>) -> Result<(), Error> {
    let participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
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

pub async fn edit_given_name(
    bot: Bot,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    update_dialogue(State::ReceiveGivenName(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_last_name(
    bot: Bot,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    update_dialogue(State::ReceiveLastName(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_gender(
    bot: Bot,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    update_dialogue(State::ReceiveGender(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_street(
    bot: Bot,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    update_dialogue(State::ReceiveStreet(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_city(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<(), Error> {
    update_dialogue(State::ReceiveCity(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_phone(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<(), Error> {
    update_dialogue(State::ReceivePhone(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_email(bot: Bot, dialogue: MyDialogue, pool: Pool<Postgres>) -> Result<(), Error> {
    update_dialogue(State::ReceiveEmail(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_status(
    bot: Bot,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    update_dialogue(State::ReceiveStatus(false), bot, dialogue, &pool).await?;
    Ok(())
}

pub async fn edit_status_related_info(
    bot: Bot,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    update_dialogue(State::ReceiveStatusRelatedInfo(false), bot, dialogue, &pool).await?;
    Ok(())
}
