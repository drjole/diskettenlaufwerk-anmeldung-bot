use crate::participant::Participant;
use crate::participant::{Gender, Status};
use crate::HandlerResult;
use sqlx::{Pool, Postgres};
use strum::IntoEnumIterator;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    Bot,
};

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveGivenName,
    ReceiveLastName,
    ReceiveGender,
    ReceiveStreet,
    ReceiveCity,
    ReceivePhone,
    ReceiveEmail,
    ReceiveStatus,
    ReceiveMatriculationNumber,
    ReceiveBusinessPhone,
}

pub async fn dialogue_start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Bitte gib deinen Vornamen ein.")
        .await?;
    dialogue.update(State::ReceiveGivenName).await?;
    Ok(())
}

pub async fn receive_given_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            sqlx::query!(
                "UPDATE participants SET given_name = $1 WHERE chat_id = $2",
                text,
                msg.chat.id.0
            )
            .execute(&pool)
            .await?;
            bot.send_message(msg.chat.id, "Bitte gib deinen Nachnamen ein.")
                .await?;
            dialogue.update(State::ReceiveLastName).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deinen Vornamen ein.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_last_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            sqlx::query!(
                "UPDATE participants SET last_name = $1 WHERE chat_id = $2",
                text,
                msg.chat.id.0
            )
            .execute(&pool)
            .await?;
            let keyboard = make_gender_keyboard();
            bot.send_message(msg.chat.id, "Bitte wähle dein Geschlecht aus.")
                .reply_markup(keyboard)
                .await?;
            dialogue.update(State::ReceiveGender).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deinen Nachnamen ein.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_gender(
    bot: Bot,
    dialogue: MyDialogue,
    q: CallbackQuery,
    pool: Pool<Postgres>,
) -> HandlerResult {
    bot.answer_callback_query(q.id).await?;
    match q.data.map(|text| text.parse::<Gender>()) {
        Some(Ok(gender)) => {
            sqlx::query!(
                "UPDATE participants SET gender = $1 WHERE chat_id = $2",
                gender as Gender,
                dialogue.chat_id().0
            )
            .execute(&pool)
            .await?;
            bot.send_message(
                dialogue.chat_id(),
                "Bitte gib deine Straße und Hausnummer ein. Z.B. Musterstr. 123",
            )
            .await?;
            dialogue.update(State::ReceiveStreet).await?;
        }
        _ => {
            let keyboard = make_gender_keyboard();
            bot.send_message(
                dialogue.chat_id(),
                "Das habe ich nicht verstanden. Bitte wähle dein Geschlecht aus.",
            )
            .reply_markup(keyboard)
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_street(
    bot: Bot,
    dialogue: MyDialogue,
    (given_name, last_name, gender): (String, String, Gender),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(
                msg.chat.id,
                "Bitte gib deine Postleitzahl und deinen Wohnort ein. Z.B. 50678 Köln",
            )
            .await?;
            dialogue.update(State::ReceiveCity).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Das habe ich nicht verstanden. Bitte gib deine Straße und Hausnummer ein. Z.B. Musterstr. 123")
                .await?;
        }
    }
    Ok(())
}

pub async fn receive_city(
    bot: Bot,
    dialogue: MyDialogue,
    (given_name, last_name, gender, street): (String, String, Gender, String),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Bitte gib deine Telefonnummer ein.")
                .await?;
            dialogue.update(State::ReceivePhone).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Das habe ich nicht verstanden. Bitte gib deine Postleitzahl und deinen Wohnort ein. Z.B. 50678 Köln")
                .await?;
        }
    }
    Ok(())
}

pub async fn receive_phone(
    bot: Bot,
    dialogue: MyDialogue,
    (given_name, last_name, gender, street, city): (String, String, Gender, String, String),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Bitte gib deine E-Mail-Adresse ein.")
                .await?;
            dialogue.update(State::ReceiveEmail).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine Telefonnummer ein.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_email(
    bot: Bot,
    dialogue: MyDialogue,
    (given_name, last_name, gender, street, city, phone): (
        String,
        String,
        Gender,
        String,
        String,
        String,
    ),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            let keyboard = make_status_keyboard();
            bot.send_message(msg.chat.id, "Bitte wähle deinen Status aus:")
                .reply_markup(keyboard)
                .await?;
            dialogue.update(State::ReceiveStatus).await?
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine E-Mail-Adresse ein.",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_status(
    bot: Bot,
    dialogue: MyDialogue,
    (given_name, last_name, gender, street, city, phone, email): (
        String,
        String,
        Gender,
        String,
        String,
        String,
        String,
    ),
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(q.id).await?;
    match q.data.map(|text| text.parse::<Status>()) {
        Some(Ok(status)) => {
            if status.is_student() {
                bot.send_message(dialogue.chat_id(), "Bitte gib deine Matrikelnummer ein.")
                    .await?;
                dialogue.update(State::ReceiveMatriculationNumber).await?;
            } else if status.is_employed_at_cgn_uni_related_thing() {
                bot.send_message(
                    dialogue.chat_id(),
                    "Bitte gib deine dienstliche Telefonnummer ein.",
                )
                .await?;
                dialogue.update(State::ReceiveBusinessPhone).await?;
            } else {
                let participant = Participant {
                    given_name,
                    last_name,
                    gender,
                    street,
                    city,
                    phone,
                    email,
                    status,
                    matriculation_number: "".into(),
                    business_phone: "".into(),
                };
                dialogue_done(bot, dialogue, participant).await?;
            }
        }
        _ => {
            let keyboard = make_status_keyboard();
            bot.send_message(
                dialogue.chat_id(),
                "Das habe ich nicht verstanden. Bitte wähle deinen Status aus:",
            )
            .reply_markup(keyboard)
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_matriculation_number(
    bot: Bot,
    dialogue: MyDialogue,
    (given_name, last_name, gender, street, city, phone, email, status): (
        String,
        String,
        Gender,
        String,
        String,
        String,
        String,
        Status,
    ),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            let participant = Participant {
                gender,
                given_name,
                last_name,
                street,
                city,
                status,
                email,
                phone,
                matriculation_number: text.into(),
                business_phone: "".into(),
            };
            dialogue_done(bot, dialogue, participant).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine Matrikelnummer an:",
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn receive_business_phone(
    bot: Bot,
    dialogue: MyDialogue,
    (given_name, last_name, gender, street, city, phone, email, status): (
        String,
        String,
        Gender,
        String,
        String,
        String,
        String,
        Status,
    ),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            let participant = Participant {
                gender,
                given_name,
                last_name,
                street,
                city,
                status,
                email,
                phone,
                matriculation_number: "".into(),
                business_phone: text.into(),
            };
            dialogue_done(bot, dialogue, participant).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "Das habe ich nicht verstanden. Bitte gib deine dienstliche Telefonnummer ein.",
            )
            .await?;
        }
    }
    Ok(())
}

async fn dialogue_done(bot: Bot, dialogue: MyDialogue, participant: Participant) -> HandlerResult {
    dbg!(participant);
    bot.send_message(dialogue.chat_id(), "Super, damit habe ich alle Daten, die ich brauche. Wenn ein neues Training ansteht, wirst du von mir benachrichtigt. Du kannst dich dann über eine kurze Antwort zum Training anmelden lassen.").await?;
    dialogue.exit().await?;
    Ok(())
}

fn make_gender_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let row = Gender::iter()
        .map(|gender| InlineKeyboardButton::callback(gender.to_string(), gender.to_string()))
        .collect();
    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}

fn make_status_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for status in Status::iter() {
        let row = vec![InlineKeyboardButton::callback(
            status.to_string(),
            status.as_str(),
        )];
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}