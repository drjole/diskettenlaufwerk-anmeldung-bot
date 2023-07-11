use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

use crate::{
    bot::{
        schema::{MyDialogue, State},
        utils::{gender_keyboard, status_keyboard, update_dialogue},
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
        update_dialogue(State::ReceiveStreet(true), bot, dialogue, &pool).await?;
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
        update_dialogue(State::ReceiveStatusRelatedInfo(true), bot, dialogue, &pool).await?;
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
