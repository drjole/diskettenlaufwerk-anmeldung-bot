use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

use crate::{
    bot::{
        schema::{MyDialogue, State},
        utils::{gender_keyboard, status_keyboard},
    },
    models::{gender::Gender, participant::Participant, status::Status},
    Error,
};

pub async fn receive_gender(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    bot.answer_callback_query(q.id).await?;
    let mut participant = Participant::find_by_chat_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Ok(gender)) = q.data.map(|text| text.parse::<Gender>()) {
        participant.gender = Some(gender);
        participant.update(&pool).await?;
        bot.send_message(
            dialogue.chat_id(),
            "Bitte gib deine Straße und Hausnummer ein. Z.B. Musterstr. 123",
        )
        .await?;
        dialogue.update(State::ReceiveStreet).await?;
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

pub async fn receive_status(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    bot.answer_callback_query(q.id).await?;
    let mut participant = Participant::find_by_chat_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Ok(status)) = q.data.map(|text| text.parse::<Status>()) {
        participant.status = Some(status.clone());
        participant.update(&pool).await?;
        if status.is_student() {
            bot.send_message(dialogue.chat_id(), "Bitte gib deine Matrikelnummer ein.")
                .await?;
            dialogue.update(State::ReceiveMatriculationNumber).await?;
        } else if status.is_employed_at_cgn_uni_related_thing() {
            bot.send_message(
                dialogue.chat_id(),
                "Bitte gib deine dienstliche Telefonnummer ein.",
            )
            .await?;
            dialogue.update(State::ReceiveBusinessPhone).await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Done!").await?;
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
