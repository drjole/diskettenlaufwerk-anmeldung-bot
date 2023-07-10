use teloxide::prelude::*;

use crate::Error;

pub async fn invalid(bot: Bot, msg: Message) -> Result<(), Error> {
    bot.send_message(msg.chat.id, "Diese Nachricht konnte ich nicht verarbeiten.")
        .await?;
    Ok(())
}
