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
};
use color_eyre::Result;
use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

pub async fn receive_gender(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!(
        "answering receive gender callback query {} from chat {}",
        q.id,
        dialogue.chat_id()
    );
    bot.answer_callback_query(q.id).await?;
    let mut participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Ok(gender)) = q.data.map(|text| text.parse::<Gender>()) {
        participant.gender = Some(gender);
        participant.update(&pool).await?;
        let state = dialogue_state(&dialogue).await;
        if state.is_in_dialogue() {
            update_dialogue(State::ReceiveStreet(true), bot, dialogue, &pool).await?;
        } else {
            bot.send_message(dialogue.chat_id(), "Geschlecht geändert.")
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

pub async fn receive_status(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!(
        "answering receive status callback query {} from chat {}",
        q.id,
        dialogue.chat_id()
    );
    bot.answer_callback_query(q.id).await?;
    let mut participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Ok(status)) = q.data.map(|text| text.parse::<Status>()) {
        participant.status = Some(status.clone());
        participant.update(&pool).await?;
        let state = dialogue_state(&dialogue).await;
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

pub async fn receive_signup_response(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<()> {
    log::info!(
        "answering receive signup callback query {} from chat {}",
        q.id,
        dialogue.chat_id()
    );
    bot.answer_callback_query(q.id).await?;
    let participant = Participant::find_by_id(&pool, dialogue.chat_id().0).await?;
    if let Some(Ok(signup_request)) = q
        .data
        .map(|text| serde_json::from_str::<SignupRequest>(&text))
    {
        if signup_request.answer {
            bot.send_message(dialogue.chat_id(), "Ok, einen Moment bitte...")
                .await?;
            signup(&participant, signup_request.course_id).await?;
            participant
                .set_signup_status(&pool, signup_request.course_id, SignupStatus::SignedUp)
                .await?;
            bot.send_message(dialogue.chat_id(), "Das sollte geklappt haben! Schau zur Sicherheit aber noch in dein E-Mail-Postfach.").await?;
        } else {
            participant
                .set_signup_status(&pool, signup_request.course_id, SignupStatus::Rejected)
                .await?;
            bot.send_message(dialogue.chat_id(), "Ok, dann vielleicht beim nächsten Mal!")
                .await?;
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

pub async fn receive_email_callback(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<()> {
    bot.answer_callback_query(q.id).await?;
    let state = dialogue_state(&dialogue).await;
    if state.is_in_dialogue() {
        update_dialogue(State::ReceiveStatus(true), bot, dialogue, &pool)
            .await
            .unwrap();
    } else {
        bot.send_message(
            dialogue.chat_id(),
            "Eingabe der E-Mail-Adresse übersprungen.",
        )
        .await?;
        dialogue.reset().await.unwrap();
    }
    Ok(())
}

pub async fn invalid_callback_query(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
) -> Result<()> {
    log::info!(
        "answering invalid callback query {} from chat {}",
        q.id,
        dialogue.chat_id()
    );
    bot.answer_callback_query(q.id).await?;
    bot.send_message(
        dialogue.chat_id(),
        "Das habe ich nicht verstanden. Versuche es mit /help.",
    )
    .await?;
    Ok(())
}
