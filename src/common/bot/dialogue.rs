use crate::{
    bot::{
        keyboards::{gender_keyboard, status_keyboard},
        schema::{MyDialogue, State},
    },
    models::participant::Participant,
};
use color_eyre::Result;
use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

use super::keyboards::no_answer_keyboard;

pub async fn dialogue_state(dialogue: &MyDialogue) -> State {
    dialogue.get().await.unwrap().unwrap()
}

pub async fn update_dialogue(
    new_state: State,
    bot: Bot,
    dialogue: MyDialogue,
    pool: &Pool<Postgres>,
) -> Result<()> {
    let participant = Participant::find_by_id(pool, dialogue.chat_id().0).await?;
    let message = match new_state {
        State::Default => "",
        State::ReceiveSignupResponse => "",
        State::ReceiveGivenName(_) => "Bitte gib deinen Vornamen ein.",
        State::ReceiveLastName(_) => "Bitte gib deinen Nachnamen ein.",
        State::ReceiveGender(_) => "Bitte wähle dein Geschlecht aus.",
        State::ReceiveStreet(_) => {
            "Bitte gib deine Straße und deine Hausnummer ein.\n\nBeispiel: Musterstr. 123"
        }
        State::ReceiveCity(_) => {
            "Bitte gib deine Postleitzahl und deinen Ort ein.\n\nBeispiel: 50678 Köln"
        }
        State::ReceivePhone(_) => "Bitte gib deine Telefonnummer ein.",
        State::ReceiveEmail(_) => "Bitte gib deine E-Mail-Adresse ein.",
        State::ReceiveStatus(_) => "Bitte wähle deinen Status aus.",
        State::ReceiveStatusRelatedInfo(_) => {
            if participant.is_student() {
                "Bitte gib deine Matrikelnummer ein."
            } else if participant.is_employed_at_cgn_uni_related_thing() {
                "Bitte gib deine dienstliche Telefonnummer ein."
            } else {
                "this is ignored later!"
            }
        }
    };

    match new_state {
        State::ReceiveGender(_) => {
            bot.send_message(dialogue.chat_id(), message)
                .reply_markup(gender_keyboard())
                .await?;
        }
        State::ReceiveEmail(_) => {
            bot.send_message(dialogue.chat_id(), message)
                .reply_markup(no_answer_keyboard())
                .await?;
        }
        State::ReceiveStatus(_) => {
            bot.send_message(dialogue.chat_id(), message)
                .reply_markup(status_keyboard())
                .await?;
        }
        State::ReceiveStatusRelatedInfo(_) => {
            if participant.status.is_some() {
                bot.send_message(dialogue.chat_id(), message).await?;
            } else {
                bot.send_message(
                    dialogue.chat_id(),
                    "Du musst zuerst deinen Status auswählen: /edit_status",
                )
                .await?;
                return Ok(());
            }
        }
        _ => {
            bot.send_message(dialogue.chat_id(), message).await?;
        }
    };

    dialogue.update(new_state).await.unwrap();

    Ok(())
}
