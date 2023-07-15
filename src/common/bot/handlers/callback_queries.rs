use crate::{
    bot::{
        dialogue::{dialogue_state, update_dialogue},
        keyboards::{gender_keyboard, status_keyboard},
        schema::{MyDialogue, State},
    },
    models::{
        gender::Gender,
        participant::Participant,
        signup::{signup, SignupRequest, SignupStatus},
        status::Status,
    },
    types::Error,
};
use color_eyre::eyre::eyre;
use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

pub async fn receive_gender(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    bot.answer_callback_query(q.id).await?;
    let mut participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Ok(gender)) = q.data.map(|text| text.parse::<Gender>()) {
        participant.gender = Some(gender);
        participant.update(&pool).await?;
        let state = dialogue_state(&dialogue, &bot).await?;
        if state.is_in_dialogue() {
            update_dialogue(State::ReceiveStreet(true), bot, dialogue, &pool).await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Geschlecht geändert.")
                .await?;
            dialogue.reset().await?;
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

pub async fn receive_status(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    bot.answer_callback_query(q.id).await?;
    let mut participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Ok(status)) = q.data.map(|text| text.parse::<Status>()) {
        participant.status = Some(status.clone());
        participant.update(&pool).await?;
        let state = dialogue_state(&dialogue, &bot).await?;
        if state.is_in_dialogue() {
            update_dialogue(State::ReceiveStatusRelatedInfo(true), bot, dialogue, &pool).await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Status geändert.")
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

pub async fn receive_signup(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    bot.answer_callback_query(q.id).await?;
    let participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Ok(signup_request)) = q
        .data
        .map(|text| serde_json::from_str::<SignupRequest>(&text))
    {
        signup(&participant, signup_request.course_id)
            .await
            .map_err(|err| eyre!("{err}"))?;
        participant
            .set_signup_status(&pool, signup_request.course_id, SignupStatus::SignedUp)
            .await?;
    }
    Ok(())
}
