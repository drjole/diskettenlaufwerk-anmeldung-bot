use crate::models::{gender::Gender, signup::SignupRequest, status::Status};
use strum::{EnumProperty, IntoEnumIterator};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn gender_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let row = Gender::iter()
        .map(|gender| {
            InlineKeyboardButton::callback(
                gender
                    .get_str("pretty")
                    .unwrap_or_else(|| panic!("Better set that enum prop")),
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
        let row = vec![InlineKeyboardButton::callback(
            status
                .get_str("pretty")
                .unwrap_or_else(|| panic!("Better set that enum prop")),
            status.to_string(),
        )];
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn signup_keyboard(course_id: i64) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let yes_answer: SignupRequest = SignupRequest {
        course_id,
        answer: true,
    };
    let no_answer: SignupRequest = SignupRequest {
        course_id,
        answer: false,
    };

    let row = vec![
        InlineKeyboardButton::callback(
            "Aber sowas von!",
            serde_json::to_string(&yes_answer).unwrap_or_else(|e| panic!("{e}")),
        ),
        InlineKeyboardButton::callback(
            "Heute nicht.",
            serde_json::to_string(&no_answer).unwrap_or_else(|e| panic!("{e}")),
        ),
    ];
    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}
