use teloxide::{prelude::*, utils::command::BotCommand};

use rand::{thread_rng, Rng};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "be a cat.")]
    Meow,
    #[command(description = "generate a random number within [0; 1).")]
    Generate,
}

fn generate() -> String {
    thread_rng().gen_range(0.0, 1.0).to_string()
}

async fn answer(
    cx: UpdateWithCx<Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Generate => cx.answer(generate()).send().await?,
        Command::Meow => cx.answer("I am a cat! Meow!").send().await?,
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot!");

    let bot = Bot::from_env();

    polling_default(bot)
        .basic_config()
        // Only iterate through commands in a proper format:
        .commands::<Command, &str>(panic!("Insert here your bot's name"))
        .for_each_concurrent(None, |(cx, command, _)| async move {
            answer(cx, command).await.log_on_error().await;
        })
        .await;
}
