use crate::bot::schema::{MyDialogue, State};
use crate::models::{gender::Gender, status::Status};
use color_eyre::eyre::Result;
use strum::{EnumProperty, IntoEnumIterator};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn update_dialogue(
    new_state: State,
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> Result<()> {
    let message = match new_state {
        State::Start => todo!(),
        State::ReceiveGivenName => "Bitte gib deinen Vornamen ein.",
        State::ReceiveLastName => "Bitte gib deinen Nachnamen ein.",
        State::ReceiveGender => "Bitte wähle dein Geschlecht aus.",
        State::ReceiveStreet => "Bitte gib deine Straße ein\n\nBeispiel: Musterstr. 123",
        State::ReceiveCity => {
            "Bitte gib deinen Ort und deine Postleitzahl ein.\n\nBeispiel: 50678 Köln"
        }
        State::ReceivePhone => "Bitte gib deine Telefonnummer ein.",
        State::ReceiveEmail => "Bitte gib deine E-Mail-Adresse ein.",
        State::ReceiveStatus => "Bitte wähle deinen Status aus.",
        State::ReceiveMatriculationNumber => "Bitte gib deinen Matrikelnummer ein.",
        State::ReceiveBusinessPhone => "Bitte gib deine dienstliche Telefonnummer ein.",
    };
    match new_state {
        State::ReceiveGender => {
            bot.send_message(msg.chat.id, message)
                .reply_markup(gender_keyboard())
                .await?
        }
        State::ReceiveStatus => {
            bot.send_message(msg.chat.id, message)
                .reply_markup(status_keyboard())
                .await?
        }
        _ => bot.send_message(msg.chat.id, message).await?,
    };
    dialogue.update(new_state).await?;
    Ok(())
}

pub fn gender_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    #[allow(clippy::expect_used)]
    let row = Gender::iter()
        .map(|gender| {
            InlineKeyboardButton::callback(
                gender.get_str("pretty").expect("Better add that enum prop"),
                gender.to_string(),
            )
        })
        .collect();
    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn status_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for status in Status::iter().rev() {
        #[allow(clippy::expect_used)]
        let row = vec![InlineKeyboardButton::callback(
            status.get_str("pretty").expect("Better add that enum prop"),
            status.to_string(),
        )];
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
