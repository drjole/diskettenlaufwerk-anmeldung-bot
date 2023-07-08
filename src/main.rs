mod courses;
mod http;
mod my_dialogue;
mod participant;
mod signup;

extern crate encoding;

use dotenv::dotenv;
use my_dialogue::{
    dialogue_start, receive_business_phone, receive_city, receive_email, receive_gender,
    receive_given_name, receive_last_name, receive_matriculation_number, receive_phone,
    receive_status, receive_street, MyDialogue, State,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    dptree::{case, deps, endpoint},
    prelude::*,
    utils::command::BotCommands,
};

type HandlerResult = Result<(), Error>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(BotCommands, Clone)]
#[command(description = "Diese Befehle sind verfügbar:")]
enum Command {
    #[command(rename = "eingeben", description = "Persönliche Daten eingeben")]
    EnterData,
    #[command(rename = "anzeigen", description = "Persönliche Daten anzeigen")]
    ShowData,
}

#[tokio::main]
async fn main() -> HandlerResult {
    dotenv()?;

    pretty_env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let bot = Bot::from_env();
    bot.set_my_commands(Command::bot_commands()).await?;
    Dispatcher::builder(bot, schema())
        .dependencies(deps![InMemStorage::<State>::new(), pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

fn schema() -> UpdateHandler<Error> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::ShowData].endpoint(show_data))
        .branch(case![Command::EnterData].endpoint(enter_data));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Start].endpoint(dialogue_start))
        .branch(case![State::ReceiveGivenName].endpoint(receive_given_name))
        .branch(case![State::ReceiveLastName].endpoint(receive_last_name))
        .branch(case![State::ReceiveStreet].endpoint(receive_street))
        .branch(case![State::ReceiveCity].endpoint(receive_city))
        .branch(case![State::ReceivePhone].endpoint(receive_phone))
        .branch(case![State::ReceiveEmail].endpoint(receive_email))
        .branch(case![State::ReceiveMatriculationNumber].endpoint(receive_matriculation_number))
        .branch(case![State::ReceiveBusinessPhone].endpoint(receive_business_phone))
        .branch(endpoint(invalid));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::ReceiveGender].endpoint(receive_gender))
        .branch(case![State::ReceiveStatus].endpoint(receive_status));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn show_data(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "data").await?;
    Ok(())
}

async fn enter_data(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    pool: Pool<Postgres>,
) -> HandlerResult {
    sqlx::query!(
        "INSERT INTO participants (chat_id) VALUES ($1)",
        msg.chat.id.0
    )
    .execute(&pool)
    .await?;

    bot.send_message(msg.chat.id, "Bitte gib deinen Vornamen ein.")
        .await?;
    dialogue.update(State::ReceiveGivenName).await?;
    Ok(())
}

async fn invalid(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Das habe ich nicht verstanden.")
        .await?;
    Ok(())
}
