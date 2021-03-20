<div align="center">
  <img src="ICON.png" width="250"/>
  <h1>teloxide</h1>
  <a href="https://docs.rs/teloxide/">
    <img src="https://docs.rs/teloxide/badge.svg">
  </a>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://teloxide.netlify.com">
    <img src="https://img.shields.io/badge/docs-dev-blue)">
  </a>
  <a href="https://crates.io/crates/teloxide">
    <img src="https://img.shields.io/crates/v/teloxide.svg">
  </a>
  <a href="https://core.telegram.org/bots/api">
    <img src="https://img.shields.io/badge/API coverage-Up to 5.1 (inclusively)-green.svg">
  </a>
  <a href="https://t.me/teloxide">
    <img src="https://img.shields.io/badge/official%20chat-t.me%2Fteloxide-blueviolet">
  </a>

  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Table of contents
 - [Highlights](#highlights)
 - [Setting up your environment](#setting-up-your-environment)
 - [API overview](#api-overview)
   - [The dices bot](#the-dices-bot)
   - [Commands](#commands)
   - [Dialogues management](#dialogues-management)
 - [Recommendations](#recommendations)
 - [Cargo features](#cargo-features)
 - [FAQ](#faq)
 - [Community bots](#community-bots)
 - [Contributing](#contributing)

## Highlights

 - **Functional reactive design.** teloxide follows [functional reactive design], allowing you to declaratively manipulate streams of updates from Telegram using filters, maps, folds, zips, and a lot of [other adaptors].

[functional reactive design]: https://en.wikipedia.org/wiki/Functional_reactive_programming
[other adaptors]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html

 - **Dialogues management subsystem.** We have designed our dialogues management subsystem to be easy-to-use, and, furthermore, to be agnostic of how/where dialogues are stored. For example, you can just replace a one line to achieve [persistence]. Out-of-the-box storages include [Redis] and [Sqlite].

[persistence]: https://en.wikipedia.org/wiki/Persistence_(computer_science)
[Redis]: https://redis.io/
[Sqlite]: https://www.sqlite.org

 - **Strongly typed bot commands.** You can describe bot commands as enumerations, and then they'll be automatically constructed from strings — just like JSON structures in [serde-json] and command-line arguments in [structopt].

[structopt]: https://github.com/TeXitoi/structopt
[serde-json]: https://github.com/serde-rs/json

## Setting up your environment
 1. [Download Rust](http://rustup.rs/).
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 3. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows
$ set TELOXIDE_TOKEN=<Your token here>
```
 4. Make sure that your Rust compiler is up to date:
```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```

 5. Run `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:
```toml
[dependencies]
teloxide = "0.4"
log = "0.4.8"
pretty_env_logger = "0.4.0"
tokio = { version =  "1.3", features = ["rt-threaded", "macros"] }
```

## API overview

### The dices bot
This bot replies with a dice throw to each received message:

([Full](./examples/dices_bot/src/main.rs))
```rust,no_run
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().await?;
        respond(())
    })
    .await;
}
```

<div align="center">
  <kbd>
    <img src=../../raw/master/media/DICES_BOT.gif />
  </kbd>
</div>

### Commands
Commands are strongly typed and defined declaratively, similar to how we define CLI using [structopt] and JSON structures in [serde-json]. The following bot accepts these commands:

 - `/username <your username>`
 - `/usernameandage <your username> <your age>`
 - `/help`

[structopt]: https://docs.rs/structopt/0.3.9/structopt/
[serde-json]: https://github.com/serde-rs/json

([Full](./examples/simple_commands_bot/src/main.rs))
```rust,no_run
use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Username(username) => {
            cx.answer(format!("Your username is @{}.", username)).await?
        }
        Command::UsernameAndAge { username, age } => {
            cx.answer(format!("Your username is @{} and age is {}.", username, age)).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = panic!("Your bot's name here");
    teloxide::commands_repl(bot, bot_name, answer).await;
}
```

<div align="center">
  <kbd>
    <img src=../../raw/master/media/SIMPLE_COMMANDS_BOT.gif />
  </kbd>
</div>

### Dialogues management
A dialogue is described by an enumeration where each variant is one of possible dialogue's states. There are also _subtransition functions_, which turn a dialogue from one state to another, thereby forming a [FSM].

[FSM]: https://en.wikipedia.org/wiki/Finite-state_machine

Below is a bot that asks you three questions and then sends the answers back to you. First, let's start with an enumeration (a collection of our dialogue's states):

([dialogue_bot/src/dialogue/mod.rs](./examples/dialogue_bot/src/dialogue/mod.rs))
```rust,ignore
// Imports are omitted...

#[derive(Transition, From)]
pub enum Dialogue {
    Start(StartState),
    ReceiveFullName(ReceiveFullNameState),
    ReceiveAge(ReceiveAgeState),
    ReceiveLocation(ReceiveLocationState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
```

When a user sends a message to our bot and such a dialogue does not exist yet, a `Dialogue::default()` is invoked, which is a `Dialogue::Start` in this case. Every time a message is received, an associated dialogue is extracted and then passed to a corresponding subtransition function:

<details>
  <summary>Dialogue::Start</summary>

([dialogue_bot/src/dialogue/states/start.rs](./examples/dialogue_bot/src/dialogue/states/start.rs))
```rust,ignore
// Imports are omitted...

pub struct StartState;

#[teloxide(subtransition)]
async fn start(
    _state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("Let's start! What's your full name?").await?;
    next(ReceiveFullNameState)
}
```

</details>

<details>
  <summary>Dialogue::ReceiveFullName</summary>

([dialogue_bot/src/dialogue/states/receive_full_name.rs](./examples/dialogue_bot/src/dialogue/states/receive_full_name.rs))
```rust,ignore
// Imports are omitted...

#[derive(Generic)]
pub struct ReceiveFullNameState;

#[teloxide(subtransition)]
async fn receive_full_name(
    state: ReceiveFullNameState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("How old are you?").await?;
    next(ReceiveAgeState::up(state, ans))
}
```

</details>

<details>
  <summary>Dialogue::ReceiveAge</summary>

([dialogue_bot/src/dialogue/states/receive_age.rs](./examples/dialogue_bot/src/dialogue/states/receive_age.rs))
```rust,ignore
// Imports are omitted...

#[derive(Generic)]
pub struct ReceiveAgeState {
    pub full_name: String,
}

#[teloxide(subtransition)]
async fn receive_age_state(
    state: ReceiveAgeState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.parse::<u8>() {
        Ok(ans) => {
            cx.answer("What's your location?").await?;
            next(ReceiveLocationState::up(state, ans))
        }
        _ => {
            cx.answer("Send me a number.").await?;
            next(state)
        }
    }
}
```

</details>

<details>
    <summary>Dialogue::ReceiveLocation</summary>

([dialogue_bot/src/dialogue/states/receive_location.rs](./examples/dialogue_bot/src/dialogue/states/receive_location.rs))
```rust,ignore
// Imports are omitted...

#[derive(Generic)]
pub struct ReceiveLocationState {
    pub full_name: String,
    pub age: u8,
}

#[teloxide(subtransition)]
async fn receive_location(
    state: ReceiveLocationState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer(format!("Full name: {}\nAge: {}\nLocation: {}", state.full_name, state.age, ans))
        .await?;
    exit()
}
```

</details>

All these subtransition functions accept a corresponding state (one of the many variants of `Dialogue`), a context, and a textual message. They return `TransitionOut<Dialogue>`, e.g. a mapping from `<your state type>` to `Dialogue`.

Finally, the `main` function looks like this:

([dialogue_bot/src/main.rs](./examples/dialogue_bot/src/main.rs))
```rust,ignore
// Imports are omitted...

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::dialogues_repl(bot, |message, dialogue| async move {
        handle_message(message, dialogue).await.expect("Something wrong with the bot!")
    })
    .await;
}

async fn handle_message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("Send me a text message.").await?;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}
```

<div align="center">
  <kbd>
    <img src=../../raw/master/media/DIALOGUE_BOT.gif />
  </kbd>
</div>

[More examples!](./examples)

## Recommendations
 - Use this pattern:
 
 ```rust
 #[tokio::main]
 async fn main() {
     run().await;
 }
 
 async fn run() {
     // Your logic here...
 }
 ```
 
 Instead of this:
 
 ```rust
#[tokio::main]
 async fn main() {
     // Your logic here...
 }
 ```

The second one produces very strange compiler messages due to the `#[tokio::main]` macro. However, the examples in this README use the second variant for brevity.

## Cargo features

 - `redis-storage` -- enables the [Redis] support.
 - `sqlite-storage` -- enables the [Sqlite] support.
 - `cbor-serializer` -- enables the [CBOR] serializer for dialogues.
 - `bincode-serializer` -- enables the [Bincode] serializer for dialogues.
 - `frunk` -- enables [`teloxide::utils::UpState`], which allows mapping from a structure of `field1, ..., fieldN` to a structure of `field1, ..., fieldN, fieldN+1`.
 - `macros` -- re-exports macros from [`teloxide-macros`].
 - `native-tls` -- enables the [`native-tls`] TLS implementation (enabled by default).
 - `rustls` -- enables the [`rustls`] TLS implementation.
 - `auto-send` -- enables `AutoSend` bot adaptor.
 - `cache-me` -- enables the `CacheMe` bot adaptor.
 - `full` -- enables all the features except `nightly`.
 - `nightly` -- enables nightly-only features (see the [teloxide-core's features]).

[CBOR]: https://en.wikipedia.org/wiki/CBOR
[Bincode]: https://github.com/servo/bincode
[`teloxide::utils::UpState`]: https://docs.rs/teloxide/latest/teloxide/utils/trait.UpState.html
[`teloxide-macros`]: https://github.com/teloxide/teloxide-macros
[`native-tls`]: https://docs.rs/native-tls
[`rustls`]: https://docs.rs/rustls
[teloxide-core's features]: https://docs.rs/teloxide-core/0.2.1/teloxide_core/#cargo-features

## FAQ
**Q: Where I can ask questions?**

A: [Issues](https://github.com/teloxide/teloxide/issues) is a good place for well-formed questions, for example, about:

 - the library design;
 - enhancements;
 - bug reports;
 - ...

If you can't compile your bot due to compilation errors and need quick help, feel free to ask in [our official Telegram group](https://t.me/teloxide).

**Q: Do you support the Telegram API for clients?**

A: No, only the bots API.

**Q: Why Rust?**

A: Most programming languages have their own implementations of Telegram bots frameworks, so why not Rust? We think Rust provides a good enough ecosystem and the language for it to be suitable for writing bots.

UPD: The current design relies on wide and deep trait bounds, thereby increasing cognitive complexity. It can be avoided using [mux-stream], but currently the stable Rust channel doesn't support necessary features to use [mux-stream] conveniently. Furthermore, the [mux-stream] could help to make a library out of teloxide, not a framework, since the design in this case could be defined by just combining streams of updates.

[mux-stream]: https://github.com/Hirrolot/mux-stream

**Q: Can I use webhooks?**

A: teloxide doesn't provide special API for working with webhooks due to their nature with lots of subtle settings. Instead, you should setup your webhook by yourself, as shown in [`examples/ngrok_ping_pong_bot`](./examples/ngrok_ping_pong_bot/src/main.rs) and [`examples/heroku_ping_pong_bot`](./examples/heroku_ping_pong_bot/src/main.rs).

Associated links:
 - [Marvin's Marvellous Guide to All Things Webhook](https://core.telegram.org/bots/webhooks)
 - [Using self-signed certificates](https://core.telegram.org/bots/self-signed)

**Q: Can I use different loggers?**

A: Yes. You can setup any logger, for example, [fern], e.g. teloxide has no specific requirements as it depends only on [log]. Remember that [`enable_logging!`] and [`enable_logging_with_filter!`] are just **optional** utilities.

[fern]: https://crates.io/crates/fern
[log]: https://crates.io/crates/log
[`enable_logging!`]: https://docs.rs/teloxide/latest/teloxide/macro.enable_logging.html
[`enable_logging_with_filter!`]: https://docs.rs/teloxide/latest/teloxide/macro.enable_logging_with_filter.html

## Community bots
Feel free to push your own bot into our collection!

 - [_steadylearner/subreddit_reader_](https://github.com/steadylearner/Rust-Full-Stack/tree/master/commits/teloxide/subreddit_reader)
 - [_ArtHome12/vzmuinebot -- Telegram bot for food menu navigate_](https://github.com/ArtHome12/vzmuinebot)
 - [_Hermitter/tepe -- A CLI to command a bot to send messages and files over Telegram_](https://github.com/Hermitter/tepe)
 - [_ArtHome12/cognito_bot -- The bot is designed to anonymize messages to a group_](https://github.com/ArtHome12/cognito_bot)
 - [_GoldsteinE/tg-vimhelpbot -- Link `:help` for Vim in Telegram_](https://github.com/GoldsteinE/tg-vimhelpbot)
 - [_sschiz/janitor-bot_ --  A bot that removes users trying to join to a chat that is designed for comments](https://github.com/sschiz/janitor-bot)
 - [ myblackbeard/basketball-betting-bot -- The bot lets you bet on NBA games against your buddies](https://github.com/myblackbeard/basketball-betting-bot)

## Contributing
See [CONRIBUTING.md](https://github.com/teloxide/teloxide/blob/master/CONTRIBUTING.md).
