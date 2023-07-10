use sqlx::{Pool, Postgres};
use teloxide::prelude::*;

use crate::{
    bot::schema::{MyDialogue, State},
    models::participant::Participant,
    Error,
};

pub async fn enter_data(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> Result<(), Error> {
    if let Ok(participant) = Participant::find_by_chat_id(&pool, msg.chat.id.0).await {
        participant.delete(&pool).await?;
    }
    let participant = Participant {
        chat_id: msg.chat.id.0,
        ..Default::default()
    };
    participant.insert(&pool).await?;

    bot.send_message(msg.chat.id, "Bitte gib deinen Vornamen ein.")
        .await?;
    dialogue.update(State::ReceiveGivenName).await?;
    Ok(())
}

pub async fn show_data(bot: Bot, msg: Message, pool: Pool<Postgres>) -> Result<(), Error> {
    let participant = Participant::find_by_chat_id(&pool, msg.chat.id.0).await?;
    bot.send_message(
        msg.chat.id,
        format!(
            r#"Ich habe folgende Informationen über dich gespeichert:

{participant}"#
        ),
    )
    .await?;
    Ok(())
}
