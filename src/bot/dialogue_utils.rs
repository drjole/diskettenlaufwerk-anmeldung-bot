use crate::{
    bot::{
        keyboards,
        schema::{MyDialogue, State},
        text_messages::TextMessage,
    },
    models::{course::Course, participant::Participant},
};
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};
use teloxide::{prelude::*, types::KeyboardRemove};

pub async fn state(dialogue: &MyDialogue) -> Result<State> {
    dialogue
        .get()
        .await
        .map_err(|e| eyre!(e))?
        .ok_or_else(|| eyre!("no state found"))
}

pub async fn update(
    mut new_state: State,
    bot: Bot,
    dialogue: MyDialogue,
    pool: &Pool<Postgres>,
) -> Result<()> {
    let participant = Participant::find_by_id(pool, dialogue.chat_id().0).await?;
    let message: String = match new_state {
        State::Default => String::new(),
        State::ReceiveSignupResponse(course_id) => {
            let course = Course::find_by_id(pool, course_id)
                .await?
                .ok_or_else(|| eyre!("course with id {} not found", course_id))?;
            TextMessage::SignupResponse(course).to_string()
        }
        State::ReceiveGivenName(_) => "Bitte gib deinen Vornamen ein.".into(),
        State::ReceiveLastName(_) => "Bitte gib deinen Nachnamen ein.".into(),
        State::ReceiveGender(_) => "Bitte wähle dein Geschlecht aus.".into(),
        State::ReceiveStreet(_) => {
            "Bitte gib deine Straße und deine Hausnummer ein.\n\nBeispiel: Musterstr. 123".into()
        }
        State::ReceiveCity(_) => {
            "Bitte gib deine Postleitzahl und deinen Ort ein.\n\nBeispiel: 50678 Köln".into()
        }
        State::ReceivePhone(_) => "Bitte gib deine Telefonnummer ein.".into(),
        State::ReceiveEmail(_, _) => "Bitte gib deine E-Mail-Adresse ein.".into(),
        State::ReceiveStatus(_) => "Bitte wähle deinen Status aus.".into(),
        State::ReceiveStatusInfo(_) => {
            if participant.is_student() {
                "Bitte gib deine Matrikelnummer ein.".into()
            } else if participant.is_employed_at_cgn_uni_related_thing() {
                "Bitte gib deine dienstliche Telefonnummer ein.".into()
            } else {
                "this is ignored later!".into()
            }
        }
        State::ReceiveDeleteConfirmation => {
            "Bist du sicher? Antworte mit \"JA\", um deine Daten endgültig zu löschen.".into()
        }
    };

    match new_state {
        State::ReceiveGender(_) => {
            bot.send_message(dialogue.chat_id(), message)
                .reply_markup(keyboards::gender())
                .await?;
        }
        State::ReceiveEmail(in_dialogue, _) => {
            let msg = bot
                .send_message(dialogue.chat_id(), message)
                .reply_markup(keyboards::no_answer())
                .await?;
            new_state = State::ReceiveEmail(in_dialogue, Some(msg.id));
        }
        State::ReceiveStatus(_) => {
            bot.send_message(dialogue.chat_id(), message)
                .reply_markup(keyboards::status())
                .await?;
        }
        State::ReceiveStatusInfo(_) => {
            if participant.status.is_some() {
                bot.send_message(dialogue.chat_id(), message)
                    .reply_markup(KeyboardRemove::default())
                    .await?;
            } else {
                bot.send_message(
                    dialogue.chat_id(),
                    "Du musst zuerst deinen Status auswählen: /edit_status",
                )
                .reply_markup(KeyboardRemove::default())
                .await?;
                return Ok(());
            }
        }
        State::ReceiveSignupResponse(_) => {
            bot.send_message(dialogue.chat_id(), message)
                .reply_markup(keyboards::signup())
                .await?;
        }
        _ => {
            bot.send_message(dialogue.chat_id(), message)
                .reply_markup(KeyboardRemove::default())
                .await?;
        }
    };

    dialogue.update(new_state).await.map_err(|e| eyre!(e))?;

    Ok(())
}
