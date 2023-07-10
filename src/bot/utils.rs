use crate::models::{gender::Gender, status::Status};
use strum::IntoEnumIterator;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn gender_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let row = Gender::iter()
        .map(|gender| InlineKeyboardButton::callback(gender.to_string(), gender.to_string()))
        .collect();
    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn status_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for status in Status::iter() {
        let row = vec![InlineKeyboardButton::callback(
            status.to_string(),
            status.to_string(),
        )];
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
