use crate::bot::handlers;
use color_eyre::Result;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use teloxide::{
    dispatching::{
        dialogue::{self, serializer::Bincode, ErasedStorage, RedisStorage, Storage},
        UpdateHandler,
    },
    prelude::*,
    types::MessageId,
    utils::command::BotCommands,
};

pub type MyDialogue = Dialogue<State, ErasedStorage<State>>;
pub type MyStorage = std::sync::Arc<ErasedStorage<State>>;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Default,
    ReceiveGivenName(bool),
    ReceiveLastName(bool),
    ReceiveGender(bool),
    ReceiveStreet(bool),
    ReceiveCity(bool),
    ReceivePhone(bool),
    ReceiveEmail(bool, Option<MessageId>),
    ReceiveStatus(bool),
    ReceiveStatusInfo(bool),
    ReceiveSignupResponse(i64),
    ReceiveDeleteConfirmation,
}

impl State {
    pub const fn is_in_dialogue(&self) -> bool {
        *match self {
            Self::ReceiveGivenName(in_dialogue)
            | Self::ReceiveLastName(in_dialogue)
            | Self::ReceiveGender(in_dialogue)
            | Self::ReceiveStreet(in_dialogue)
            | Self::ReceiveCity(in_dialogue)
            | Self::ReceivePhone(in_dialogue)
            | Self::ReceiveEmail(in_dialogue, _)
            | Self::ReceiveStatus(in_dialogue)
            | Self::ReceiveStatusInfo(in_dialogue) => in_dialogue,
            Self::Default | Self::ReceiveSignupResponse(_) | Self::ReceiveDeleteConfirmation => {
                &false
            }
        }
    }
}

#[derive(BotCommands, Clone, Debug)]
#[command(
    description = "Diese Befehle sind verfügbar:",
    rename_rule = "snake_case"
)]
pub enum Command {
    #[command(description = "Daten anzeigen/bearbeiten")]
    ShowData,
    #[command(description = "Anmeldung starten")]
    Signup,
    #[command(description = "Daten eingeben")]
    EnterData,
    #[command(description = "Daten löschen")]
    Delete,
    #[command(description = "Aktuelle Aktion abbrechen")]
    Cancel,
    #[command(description = "Hilfe anzeigen")]
    Help,
    #[command(description = "Vorname ändern")]
    EditGivenName,
    #[command(description = "Nachname ändern")]
    EditLastName,
    #[command(description = "Geschlecht ändern")]
    EditGender,
    #[command(description = "Straße ändern")]
    EditStreet,
    #[command(description = "Stadt ändern")]
    EditCity,
    #[command(description = "Telefonnummer ändern")]
    EditPhone,
    #[command(description = "E-Mail-Adresse ändern")]
    EditEmail,
    #[command(description = "Status ändern")]
    EditStatus,
    #[command(description = "Matrikelnummer oder dienstliche Telefonnummer ändern")]
    EditStatusInfo,
    #[command(description = "Starttext anzeigen")]
    Start,
}

pub async fn start(pool: Pool<Postgres>, redis_url: String) -> Result<()> {
    let client = teloxide::net::default_reqwest_settings()
        .timeout(Duration::from_secs(60))
        .build()?;
    let bot = Bot::from_env_with_client(client);
    bot.set_my_commands(Command::bot_commands().into_iter().take(6))
        .await?;
    let storage: MyStorage = RedisStorage::open(redis_url, Bincode).await?.erase();
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![storage, pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

fn schema() -> UpdateHandler<color_eyre::Report> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Signup].endpoint(handlers::signup))
        .branch(case![Command::ShowData].endpoint(handlers::show_data))
        .branch(case![Command::EnterData].endpoint(handlers::enter_data))
        .branch(case![Command::Delete].endpoint(handlers::delete))
        .branch(case![Command::Cancel].endpoint(handlers::cancel))
        .branch(case![Command::Help].endpoint(handlers::help))
        .branch(case![Command::EditGivenName].endpoint(handlers::edit_given_name))
        .branch(case![Command::EditLastName].endpoint(handlers::edit_last_name))
        .branch(case![Command::EditGender].endpoint(handlers::edit_gender))
        .branch(case![Command::EditStreet].endpoint(handlers::edit_street))
        .branch(case![Command::EditCity].endpoint(handlers::edit_city))
        .branch(case![Command::EditPhone].endpoint(handlers::edit_phone))
        .branch(case![Command::EditEmail].endpoint(handlers::edit_email))
        .branch(case![Command::EditStatus].endpoint(handlers::edit_status))
        .branch(case![Command::EditStatusInfo].endpoint(handlers::edit_status_info))
        .branch(case![Command::Start].endpoint(handlers::start));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::ReceiveGivenName(in_dialogue)].endpoint(handlers::receive_given_name))
        .branch(case![State::ReceiveLastName(in_dialogue)].endpoint(handlers::receive_last_name))
        .branch(case![State::ReceiveGender(in_dialogue)].endpoint(handlers::receive_gender))
        .branch(case![State::ReceiveStreet(in_dialogue)].endpoint(handlers::receive_street))
        .branch(case![State::ReceiveCity(in_dialogue)].endpoint(handlers::receive_city))
        .branch(case![State::ReceivePhone(in_dialogue)].endpoint(handlers::receive_phone))
        .branch(
            case![State::ReceiveEmail(in_dialogue, message_id)].endpoint(handlers::receive_email),
        )
        .branch(case![State::ReceiveStatus(in_dialogue)].endpoint(handlers::receive_status))
        .branch(
            case![State::ReceiveStatusInfo(in_dialogue)].endpoint(handlers::receive_status_info),
        )
        .branch(
            case![State::ReceiveSignupResponse(course_id)]
                .endpoint(handlers::receive_signup_response),
        )
        .branch(
            case![State::ReceiveDeleteConfirmation].endpoint(handlers::receive_delete_confirmation),
        )
        .branch(dptree::endpoint(handlers::invalid));

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            case![State::ReceiveEmail(in_dialogue, message_id)]
                .endpoint(handlers::receive_email_callback),
        )
        .branch(dptree::endpoint(handlers::invalid_callback_query));

    dialogue::enter::<Update, ErasedStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
