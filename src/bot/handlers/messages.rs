use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

use crate::{
    bot::{
        schema::{MyDialogue, State},
        utils::{gender_keyboard, status_keyboard},
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
            bot.send_message(msg.chat.id, "Bitte gib deinen Nachnamen ein.")
                .await?;
            dialogue.update(State::ReceiveLastName).await?;
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
            let keyboard = gender_keyboard();
            bot.send_message(msg.chat.id, "Bitte wähle dein Geschlecht aus.")
                .reply_markup(keyboard)
                .await?;
            dialogue.update(State::ReceiveGender).await?;
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
            bot.send_message(
                msg.chat.id,
                "Bitte gib deine Postleitzahl und deinen Wohnort ein. Z.B. 50678 Köln",
            )
            .await?;
            dialogue.update(State::ReceiveCity).await?;
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
            bot.send_message(msg.chat.id, "Bitte gib deine Telefonnummer ein.")
                .await?;
            dialogue.update(State::ReceivePhone).await?;
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
            bot.send_message(msg.chat.id, "Bitte gib deine E-Mail-Adresse ein.")
                .await?;
            dialogue.update(State::ReceiveEmail).await?;
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
            let keyboard = status_keyboard();
            bot.send_message(msg.chat.id, "Bitte wähle deinen Status aus:")
                .reply_markup(keyboard)
                .await?;
            dialogue.update(State::ReceiveStatus).await?;
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

pub async fn receive_matriculation_number(
    bot: Bot,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.matriculation_number = Some(text.to_string());
            participant.update(&pool).await?;
            bot.send_message(msg.chat.id, "Done!").await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine Matrikelnummer an:",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_business_phone(
    bot: Bot,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    let mut participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.business_phone = Some(text.to_string());
            participant.update(&pool).await?;
            bot.send_message(msg.chat.id, "Done!").await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine dienstliche Telefonnummer ein.",
            )
            .await?;
        }
    }
    Ok(())
}
