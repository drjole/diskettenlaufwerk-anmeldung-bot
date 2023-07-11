use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

use crate::{
    bot::{
        schema::{MyDialogue, State},
        utils::{dialogue_state, update_dialogue},
    },
    models::participant::Participant,
    Error,
};

pub async fn receive_given_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.given_name = Some(text.to_string());
            participant.update(&pool).await?;
            update_dialogue(State::ReceiveLastName(true), bot, dialogue, &pool).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deinen Vornamen ein.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_last_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.last_name = Some(text.to_string());
            participant.update(&pool).await?;
            update_dialogue(State::ReceiveGender(true), bot, dialogue, &pool).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deinen Nachnamen ein.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_street(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.street = Some(text.to_string());
            participant.update(&pool).await?;
            update_dialogue(State::ReceiveCity(true), bot, dialogue, &pool).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Das habe ich nicht verstanden. Bitte gib deine Straße und Hausnummer ein. Z.B. Musterstr. 123")
                .await?;
        }
    }
    Ok(())
}

pub async fn receive_city(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.city = Some(text.to_string());
            participant.update(&pool).await?;
            update_dialogue(State::ReceivePhone(true), bot, dialogue, &pool).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Das habe ich nicht verstanden. Bitte gib deine Postleitzahl und deinen Wohnort ein. Z.B. 50678 Köln")
                .await?;
        }
    }
    Ok(())
}

pub async fn receive_phone(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.phone = Some(text.to_string());
            participant.update(&pool).await?;
            update_dialogue(State::ReceiveEmail(true), bot, dialogue, &pool).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine Telefonnummer ein.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_email(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.email = Some(text.to_string());
            participant.update(&pool).await?;
            update_dialogue(State::ReceiveStatus(true), bot, dialogue, &pool).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine E-Mail-Adresse ein.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_status_related_info(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    let state = dialogue_state(&dialogue, &bot).await?;
    match msg.text() {
        Some(text) => {
            participant.status_related_info = Some(text.to_string());
            participant.update(&pool).await?;
            if state.is_in_dialogue() {
                bot.send_message(msg.chat.id, r#"Super! Damit habe ich alle Daten, die ich brauche.

Wenn du deine Daten ändern willst, nutze die /edit... Befehle. Diese findest du auch, wenn du dir deine Daten mittels /show_data anzeigen lässt.

Wenn Trainings anstehen, wirst du von mir benachrichtigt. Du kannst dann antworten und dich anmelden lassen."#).await?;
                dialogue.exit().await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "{} geändert.",
                        participant.status_related_info_name().unwrap_or_default()
                    ),
                )
                .await?;
            }
        }
        None => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Das habe ich nicht verstanden. Bitte gib deine {} ein.",
                    participant.status_related_info_name().unwrap_or_default()
                ),
            )
            .await?;
        }
    }
    Ok(())
}
