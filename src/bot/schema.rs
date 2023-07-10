use color_eyre::eyre::Result;
use sqlx::{Pool, Postgres};
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

use crate::{bot::handlers, Error};

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

#[derive(BotCommands, Clone)]
#[command(
    description = "Diese Befehle sind verfügbar:",
    rename_rule = "snake_case"
)]
enum Command {
    #[command(description = "Persönliche Daten eingeben")]
    EnterData,
    #[command(description = "Persönliche Daten anzeigen")]
    ShowData,
    #[command(description = "Vorname ändern")]
    EditGivenName,
    #[command(description = "Nachname ändern")]
    EditLastName,
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
    #[command(description = "Matrikelnummer ändern")]
    EditMatriculationNumber,
    #[command(description = "Dienstliche Telefonnummer ändern")]
    EditBusinessPhone,
}

pub async fn start(pool: Pool<Postgres>) -> Result<()> {
    let bot = Bot::from_env();
    bot.set_my_commands(Command::bot_commands()).await?;
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new(), pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

fn schema() -> UpdateHandler<Error> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::ShowData].endpoint(handlers::show_data))
        .branch(case![Command::EnterData].endpoint(handlers::enter_data))
        .branch(case![Command::EditGivenName].endpoint(handlers::edit_given_name))
        .branch(case![Command::EditLastName].endpoint(handlers::edit_last_name))
        .branch(case![Command::EditStreet].endpoint(handlers::edit_street))
        .branch(case![Command::EditCity].endpoint(handlers::edit_city))
        .branch(case![Command::EditPhone].endpoint(handlers::edit_phone))
        .branch(case![Command::EditEmail].endpoint(handlers::edit_email))
        .branch(case![Command::EditStatus].endpoint(handlers::edit_status))
        .branch(
            case![Command::EditMatriculationNumber].endpoint(handlers::edit_matriculation_number),
        )
        .branch(case![Command::EditBusinessPhone].endpoint(handlers::edit_business_phone));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::ReceiveGivenName].endpoint(handlers::receive_given_name))
        .branch(case![State::ReceiveLastName].endpoint(handlers::receive_last_name))
        .branch(case![State::ReceiveStreet].endpoint(handlers::receive_street))
        .branch(case![State::ReceiveCity].endpoint(handlers::receive_city))
        .branch(case![State::ReceivePhone].endpoint(handlers::receive_phone))
        .branch(case![State::ReceiveEmail].endpoint(handlers::receive_email))
        .branch(
            case![State::ReceiveMatriculationNumber]
                .endpoint(handlers::receive_matriculation_number),
        )
        .branch(case![State::ReceiveBusinessPhone].endpoint(handlers::receive_business_phone))
        .branch(dptree::endpoint(handlers::invalid));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::ReceiveGender].endpoint(handlers::receive_gender))
        .branch(case![State::ReceiveStatus].endpoint(handlers::receive_status));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
