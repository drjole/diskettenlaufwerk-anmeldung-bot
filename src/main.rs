mod courses;
mod http;
mod my_dialogue;
mod participant;
mod signup;

extern crate encoding;

use my_dialogue::{
    dialogue_start, receive_business_phone, receive_city, receive_email, receive_gender,
    receive_given_name, receive_last_name, receive_matriculation_number, receive_phone,
    receive_status, receive_street, MyDialogue, State,
};
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    dptree::{case, deps, endpoint},
    prelude::*,
    utils::command::BotCommands,
};

use dotenv::dotenv;

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
    log::info!("Starting bot...");

    let bot = Bot::from_env();
    bot.set_my_commands(Command::bot_commands()).await?;

    Dispatcher::builder(bot, schema())
        .dependencies(deps![InMemStorage::<State>::new()])
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
        .branch(case![State::ReceiveLastName { given_name }].endpoint(receive_last_name))
        .branch(
            case![State::ReceiveStreet {
                given_name,
                last_name,
                gender
            }]
            .endpoint(receive_street),
        )
        .branch(
            case![State::ReceiveCity {
                given_name,
                last_name,
                gender,
                street
            }]
            .endpoint(receive_city),
        )
        .branch(
            case![State::ReceivePhone {
                given_name,
                last_name,
                gender,
                street,
                city,
            }]
            .endpoint(receive_phone),
        )
        .branch(
            case![State::ReceiveEmail {
                given_name,
                last_name,
                gender,
                street,
                city,
                phone
            }]
            .endpoint(receive_email),
        )
        .branch(
            case![State::ReceiveMatriculationNumber {
                given_name,
                last_name,
                gender,
                street,
                city,
                phone,
                email,
                status
            }]
            .endpoint(receive_matriculation_number),
        )
        .branch(
            case![State::ReceiveBusinessPhone {
                given_name,
                last_name,
                gender,
                street,
                city,
                phone,
                email,
                status,
            }]
            .endpoint(receive_business_phone),
        )
        .branch(endpoint(invalid));

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            case![State::ReceiveGender {
                given_name,
                last_name
            }]
            .endpoint(receive_gender),
        )
        .branch(
            case![State::ReceiveStatus {
                given_name,
                last_name,
                gender,
                street,
                city,
                phone,
                email,
            }]
            .endpoint(receive_status),
        );

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn show_data(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "data").await?;
    Ok(())
}

async fn enter_data(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Bitte gib deinen Vornamen ein.")
        .await?;
    dialogue.update(State::ReceiveGivenName).await?;
    Ok(())
}

async fn invalid(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Das habe ich nicht verstanden. Versuche es mit /hilfe.",
    )
    .await?;
    Ok(())
}
