use color_eyre::Result;
use teloxide::prelude::*;

pub async fn invalid(bot: Bot, msg: Message) -> Result<()> {
    log::info!("invalid by chat {}", msg.chat.id);
    bot.send_message(msg.chat.id, "Diese Nachricht konnte ich nicht verarbeiten.")
        .await?;
    Ok(())
}
