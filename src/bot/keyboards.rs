use crate::models::{gender::Gender, signup, status::Status};
use strum::{EnumProperty, IntoEnumIterator};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};

pub fn gender_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for gender in Gender::iter() {
        let row = vec![KeyboardButton::new(
            gender
                .get_str("pretty")
                .unwrap_or_else(|| panic!("Better set that enum prop")),
        )];
        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .one_time_keyboard(true)
}

pub fn status_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for status in Status::iter() {
        let row = vec![KeyboardButton::new(
            status
                .get_str("pretty")
                .unwrap_or_else(|| panic!("Better set that enum prop")),
        )];
        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard).one_time_keyboard(true)
}

pub fn signup_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for signup_request in signup::Request::iter() {
        let row = vec![KeyboardButton::new(
            signup_request
                .get_str("pretty")
                .unwrap_or_else(|| panic!("Better set that enum prop")),
        )];
        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .one_time_keyboard(true)
}

pub fn no_answer_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let row = vec![InlineKeyboardButton::callback("Keine Angabe.", "no answer")];
    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}
