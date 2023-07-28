use crate::{
    bot::{
        dialogue_utils, keyboards,
        schema::{MyDialogue, State},
        text_messages::TextMessage,
    },
    models::{gender::Gender, participant::Participant, signup, status::Status},
};
use color_eyre::Result;
use sqlx::{Pool, Postgres};
use strum::{EnumProperty, IntoEnumIterator};
use teloxide::{prelude::*, types::KeyboardRemove};

pub async fn receive_given_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("receive_given_name by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.given_name = Some(text.to_string());
            participant.update(&pool).await?;
            let state = dialogue_utils::state(&dialogue).await;
            if state.is_in_dialogue() {
                dialogue_utils::update(State::ReceiveLastName(true), bot, dialogue, &pool).await?;
            } else {
                bot.send_message(msg.chat.id, "Vorname geändert.").await?;
                dialogue.reset().await.unwrap();
            }
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
) -> Result<()> {
    log::info!("receive_last_name by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.last_name = Some(text.to_string());
            participant.update(&pool).await?;
            let state = dialogue_utils::state(&dialogue).await;
            if state.is_in_dialogue() {
                dialogue_utils::update(State::ReceiveGender(true), bot, dialogue, &pool).await?;
            } else {
                bot.send_message(msg.chat.id, "Nachname geändert.").await?;
                dialogue.reset().await.unwrap();
            }
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

pub async fn receive_gender(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("receive_gender by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Some(gender)) = msg.text().map(|text| {
        Gender::iter().find(|g| {
            g.get_str("pretty")
                .unwrap_or_else(|| panic!("Better set that enum prop"))
                == text
        })
    }) {
        participant.gender = Some(gender);
        participant.update(&pool).await?;
        let state = dialogue_utils::state(&dialogue).await;
        if state.is_in_dialogue() {
            dialogue_utils::update(State::ReceiveStreet(true), bot, dialogue, &pool).await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Geschlecht geändert.")
                .reply_markup(KeyboardRemove::default())
                .await?;
            dialogue.reset().await.unwrap();
        }
    } else {
        let keyboard = keyboards::gender();
        bot.send_message(
            dialogue.chat_id(),
            "Das habe ich nicht verstanden. Bitte wähle dein Geschlecht aus.",
        )
        .reply_markup(keyboard)
        .await?;
    }
    Ok(())
}

pub async fn receive_street(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("receive_street by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.street = Some(text.to_string());
            participant.update(&pool).await?;
            let state = dialogue_utils::state(&dialogue).await;
            if state.is_in_dialogue() {
                dialogue_utils::update(State::ReceiveCity(true), bot, dialogue, &pool).await?;
            } else {
                bot.send_message(msg.chat.id, "Straße und Hausnummer geändert.")
                    .await?;
                dialogue.reset().await.unwrap();
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Das habe ich nicht verstanden. Bitte gib deine Straße und deine Hausnummer ein. Beispiel: Musterstr. 123")
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
) -> Result<()> {
    log::info!("receive_city by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.city = Some(text.to_string());
            participant.update(&pool).await?;
            let state = dialogue_utils::state(&dialogue).await;
            if state.is_in_dialogue() {
                dialogue_utils::update(State::ReceivePhone(true), bot, dialogue, &pool).await?;
            } else {
                bot.send_message(msg.chat.id, "Postleitzahl und Ort geändert.")
                    .await?;
                dialogue.reset().await.unwrap();
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Das habe ich nicht verstanden. Bitte gib deine Postleitzahl und deinen Ort ein.\n\nBeispiel: 50678 Köln")
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
) -> Result<()> {
    log::info!("receive_phone by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.phone = Some(text.to_string());
            participant.update(&pool).await?;
            let state = dialogue_utils::state(&dialogue).await;
            if state.is_in_dialogue() {
                dialogue_utils::update(State::ReceiveEmail(true, None), bot, dialogue, &pool)
                    .await?;
            } else {
                bot.send_message(msg.chat.id, "Telefonnummer geändert.")
                    .await?;
                dialogue.reset().await.unwrap();
            }
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
) -> Result<()> {
    log::info!("receive_email by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    match msg.text() {
        Some(text) => {
            participant.email = Some(text.to_string());
            participant.update(&pool).await?;
            let state = dialogue_utils::state(&dialogue).await;
            let message_id = match state {
                State::ReceiveEmail(_, message_id) => message_id,
                _ => None,
            }
            .unwrap();
            bot.edit_message_reply_markup(dialogue.chat_id(), message_id)
                .await?;
            if state.is_in_dialogue() {
                dialogue_utils::update(State::ReceiveStatus(true), bot, dialogue, &pool).await?;
            } else {
                bot.send_message(msg.chat.id, "E-Mail-Adresse geändert.")
                    .await?;
                dialogue.reset().await.unwrap();
            }
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine E-Mail-Adresse ein.",
            )
            .reply_markup(keyboards::no_answer())
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_status(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("receive_status by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Some(status)) = msg.text().map(|text| {
        Status::iter().find(|s| {
            s.get_str("pretty")
                .unwrap_or_else(|| panic!("Better set that enum prop"))
                == text
        })
    }) {
        participant.status = Some(status.clone());
        participant.update(&pool).await?;
        let state = dialogue_utils::state(&dialogue).await;
        if state.is_in_dialogue() {
            dialogue_utils::update(State::ReceiveStatusInfo(true), bot, dialogue, &pool).await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Status geändert.")
                .reply_markup(KeyboardRemove::default())
                .await?;
            dialogue_utils::update(State::ReceiveStatusInfo(false), bot, dialogue, &pool).await?;
        }
    } else {
        let keyboard = keyboards::status();
        bot.send_message(
            dialogue.chat_id(),
            "Das habe ich nicht verstanden. Bitte wähle deinen Status aus:",
        )
        .reply_markup(keyboard)
        .await?;
    }
    Ok(())
}

pub async fn receive_status_info(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("receive_status_info by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    let status_info_name = participant.status_info_name().unwrap_or_default();
    let state = dialogue_utils::state(&dialogue).await;
    match msg.text() {
        Some(text) => {
            participant.status_info = Some(text.to_string());
            participant.update(&pool).await?;
            if state.is_in_dialogue() {
                bot.send_message(msg.chat.id, TextMessage::EnterDataComplete.to_string())
                    .await?;
            } else {
                bot.send_message(msg.chat.id, format!("{status_info_name} geändert.",))
                    .await?;
            }
            dialogue.reset().await.unwrap();
        }
        None => {
            bot.send_message(
                msg.chat.id,
                format!("Das habe ich nicht verstanden. Bitte gib deine {status_info_name} ein.",),
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_signup_response(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("receive_signup_response by chat {}", msg.chat.id);
    let participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Some(signup_request)) = msg.text().map(|text| {
        signup::Request::iter().find(|s| {
            s.get_str("pretty")
                .unwrap_or_else(|| panic!("Better set that enum prop"))
                == text
        })
    }) {
        let course_id = match dialogue_utils::state(&dialogue).await {
            State::ReceiveSignupResponse(course_id) => Some(course_id),
            _ => None,
        }
        .unwrap();
        match signup_request {
            signup::Request::Accept => {
                bot.send_message(msg.chat.id, "Ok, einen Moment bitte...")
                    .reply_markup(KeyboardRemove::default())
                    .await?;
                match signup::perform(&participant, course_id).await {
                    Ok(_) => {
                        participant
                            .set_signup_status(&pool, course_id, signup::Status::SignedUp)
                            .await?;
                        bot.send_message(msg.chat.id, "Das hat geklappt! Wenn du deine E-Mail-Adresse angegeben hast, findest du gleich eine Bestätigung in deinem Postfach.").await?;
                    }
                    Err(err) => {
                        bot.send_message(
                            msg.chat.id,
                            format!("Fehler bei der Anmeldung:\n\n{err}"),
                        )
                        .await?;
                    }
                };
            }
            signup::Request::Reject => {
                bot.send_message(msg.chat.id, "Ok, dann vielleicht beim nächsten Mal! Solltest du dich umentscheiden, kannst du den /signup Befehl nutzen, um dich doch noch anzumelden.")
                    .reply_markup(KeyboardRemove::default())
                    .await?;
                participant
                    .set_signup_status(&pool, course_id, signup::Status::Rejected)
                    .await?;
            }
        }
        dialogue.reset().await.unwrap();
    } else {
        bot.send_message(
            dialogue.chat_id(),
            "Das habe ich nicht verstanden. Versuche es mit /help.",
        )
        .await?;
    }
    Ok(())
}

pub async fn receive_delete_confirmation(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<()> {
    match msg.text() {
        Some(text) => {
            if text == "JA" {
                let mut participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
                participant.delete(&pool).await?;
                bot.send_message(dialogue.chat_id(), "Daten gelöscht.\n\nWenn du dich wieder anmelden möchtest, nutze den /enter_data Befehl.").await?;
                dialogue.reset().await.unwrap();
            } else {
                bot.send_message(
                    dialogue.chat_id(),
                    "Deine Daten wurden <u>nicht</b> gelöscht.",
                )
                .parse_mode(teloxide::types::ParseMode::Html)
                .await?;
                dialogue.reset().await.unwrap();
            }
        }
        None => {
            bot.send_message(
                dialogue.chat_id(),
                "Das habe ich nicht verstanden. Versuche es mit /help.",
            )
            .await?;
        }
    }
    Ok(())
}
