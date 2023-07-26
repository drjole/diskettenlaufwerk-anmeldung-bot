use crate::bot::{
    dialogue::{dialogue_state, update_dialogue},
    schema::{MyDialogue, State},
};
use color_eyre::Result;
use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

pub async fn receive_email_callback(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> Result<()> {
    bot.answer_callback_query(q.id).await?;
    let message_id = match dialogue_state(&dialogue).await {
        State::ReceiveEmail(_, message_id) => message_id,
        _ => None,
    }
    .unwrap();
    bot.edit_message_reply_markup(dialogue.chat_id(), message_id)
        .await?;
    bot.send_message(
        dialogue.chat_id(),
        "Eingabe der E-Mail-Adresse übersprungen.",
    )
    .await?;
    let state = dialogue_state(&dialogue).await;
    if state.is_in_dialogue() {
        update_dialogue(State::ReceiveStatus(true), bot, dialogue, &pool)
            .await
            .unwrap();
    } else {
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
