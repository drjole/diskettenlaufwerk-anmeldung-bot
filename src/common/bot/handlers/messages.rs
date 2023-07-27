use crate::{
    bot::{
        dialogue::{dialogue_state, update_dialogue},
        keyboards::{gender_keyboard, no_answer_keyboard, status_keyboard},
        schema::{MyDialogue, State},
        text_messages::TextMessage,
    },
    models::{
        gender::Gender,
        participant::Participant,
        signup::{signup, SignupRequest, SignupStatus},
        status::Status,
    },
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
            let state = dialogue_state(&dialogue).await;
            if state.is_in_dialogue() {
                update_dialogue(State::ReceiveLastName(true), bot, dialogue, &pool).await?;
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
            let state = dialogue_state(&dialogue).await;
            if state.is_in_dialogue() {
                update_dialogue(State::ReceiveGender(true), bot, dialogue, &pool).await?;
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
        let state = dialogue_state(&dialogue).await;
        if state.is_in_dialogue() {
            update_dialogue(State::ReceiveStreet(true), bot, dialogue, &pool).await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Geschlecht geändert.")
                .reply_markup(KeyboardRemove::default())
                .await?;
            dialogue.reset().await.unwrap();
        }
    } else {
        let keyboard = gender_keyboard();
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
            let state = dialogue_state(&dialogue).await;
            if state.is_in_dialogue() {
                update_dialogue(State::ReceiveCity(true), bot, dialogue, &pool).await?;
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
            let state = dialogue_state(&dialogue).await;
            if state.is_in_dialogue() {
                update_dialogue(State::ReceivePhone(true), bot, dialogue, &pool).await?;
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
            let state = dialogue_state(&dialogue).await;
            if state.is_in_dialogue() {
                update_dialogue(State::ReceiveEmail(true, None), bot, dialogue, &pool).await?;
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
            let state = dialogue_state(&dialogue).await;
            let message_id = match state {
                State::ReceiveEmail(_, message_id) => message_id,
                _ => None,
            }
            .unwrap();
            bot.edit_message_reply_markup(dialogue.chat_id(), message_id)
                .await?;
            if state.is_in_dialogue() {
                update_dialogue(State::ReceiveStatus(true), bot, dialogue, &pool).await?;
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
            .reply_markup(no_answer_keyboard())
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
        let state = dialogue_state(&dialogue).await;
        if state.is_in_dialogue() {
            update_dialogue(State::ReceiveStatusRelatedInfo(true), bot, dialogue, &pool).await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Status geändert.")
                .reply_markup(KeyboardRemove::default())
                .await?;
            update_dialogue(State::ReceiveStatusRelatedInfo(false), bot, dialogue, &pool).await?;
        }
    } else {
        let keyboard = status_keyboard();
        bot.send_message(
            dialogue.chat_id(),
            "Das habe ich nicht verstanden. Bitte wähle deinen Status aus:",
        )
        .reply_markup(keyboard)
        .await?;
    }
    Ok(())
}

pub async fn receive_status_related_info(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!("receive_status_related_info by chat {}", msg.chat.id);
    let mut participant = Participant::find_by_id(&pool, msg.chat.id.0).await?;
    let status_related_info_name = participant.status_related_info_name().unwrap_or_default();
    let state = dialogue_state(&dialogue).await;
    match msg.text() {
        Some(text) => {
            participant.status_related_info = Some(text.to_string());
            participant.update(&pool).await?;
            if state.is_in_dialogue() {
                bot.send_message(msg.chat.id, TextMessage::EnterDataComplete.to_string())
                    .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!("{status_related_info_name} geändert.",),
                )
                .await?;
            }
            dialogue.reset().await.unwrap();
        }
        None => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Das habe ich nicht verstanden. Bitte gib deine {status_related_info_name} ein.",
                ),
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
        SignupRequest::iter().find(|s| {
            s.get_str("pretty")
                .unwrap_or_else(|| panic!("Better set that enum prop"))
                == text
        })
    }) {
        let course_id = match dialogue_state(&dialogue).await {
            State::ReceiveSignupResponse(course_id) => Some(course_id),
            _ => None,
        }
        .unwrap();
        match signup_request {
            SignupRequest::Accept => {
                bot.send_message(msg.chat.id, "Ok, einen Moment bitte...")
                    .reply_markup(KeyboardRemove::default())
                    .await?;
                match signup(&participant, course_id).await {
                    Ok(_) => {
                        participant
                            .set_signup_status(&pool, course_id, SignupStatus::SignedUp)
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
            SignupRequest::Reject => {
                bot.send_message(msg.chat.id, "Ok, dann vielleicht beim nächsten Mal! Solltest du dich umentscheiden, kannst du den /signup Befehl nutzen, um dich doch noch anzumelden.")
                    .reply_markup(KeyboardRemove::default())
                    .await?;
                participant
                    .set_signup_status(&pool, course_id, SignupStatus::Rejected)
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
