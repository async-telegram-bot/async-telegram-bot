<div align="center">
  <img src="ICON.png" width="250"/>
  <h1>teloxide</h1>
  
  <a href="https://docs.rs/teloxide/">
    <img src="https://img.shields.io/badge/docs.rs-v0.2.0-blue.svg">
  </a>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://crates.io/crates/teloxide">
    <img src="https://img.shields.io/badge/crates.io-v0.2.0-orange.svg">
  </a>
  <a href="https://t.me/teloxide">
    <img src="https://img.shields.io/badge/official%20chat-t.me%2Fteloxide-blueviolet">
  </a>
  <a href="https://core.telegram.org/bots/api">
    <img src="https://img.shields.io/badge/API coverage-Up to 0.4.6 (inclusively)-green.svg">
  </a>
  
  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Table of contents
 - [Features](https://github.com/teloxide/teloxide#features)
 - [Getting started](https://github.com/teloxide/teloxide#getting-started)
 - [Examples](https://github.com/teloxide/teloxide#examples)
   - [The ping-pong bot](https://github.com/teloxide/teloxide#the-ping-pong-bot)
   - [Commands](https://github.com/teloxide/teloxide#commands)
   - [Guess a number](https://github.com/teloxide/teloxide#guess-a-number)
 - [More examples!](https://github.com/teloxide/teloxide#more-examples)
 - [Recommendations](https://github.com/teloxide/teloxide#recommendations)
 - [FAQ](https://github.com/teloxide/teloxide#faq)
   - [Where I can ask questions?](https://github.com/teloxide/teloxide#where-i-can-ask-questions)
   - [Why Rust?](https://github.com/teloxide/teloxide#why-rust)
   - [Can I use different loggers?](https://github.com/teloxide/teloxide#can-i-use-different-loggers)
 - [Community bots](https://github.com/teloxide/teloxide#community-bots)
 - [Contributing](https://github.com/teloxide/teloxide#contributing)

## Features
<h3 align="center">Higher-order design</h3>
<p align="center">
teloxide supports <a href="https://en.wikipedia.org/wiki/Higher-order_programming">higher-order programming</a> by making <a href="https://docs.rs/futures/latest/futures/prelude/trait.Stream.html">streams</a> a <a href="https://en.wikipedia.org/wiki/First-class_citizen">first-class citizen</a>: feel free to (de)multiplex them, apply arbitrary transformations, pass to/return from other functions, <a href="https://en.wikipedia.org/wiki/Lazy_evaluation">lazily evaluate them</a>, concurrently process their items, and much more, thereby achieving extremely flexible design.
</p>

<hr>

<h3 align="center">Type safety</h3>
<p align="center">
All the API <a href="https://docs.rs/teloxide/latest/teloxide/types/index.html">types</a> and <a href="https://docs.rs/teloxide/0.2.0/teloxide/requests/index.html">methods</a> are implemented with heavy use of <a href="https://en.wikipedia.org/wiki/Algebraic_data_type"><strong>ADT</strong>s</a> to enforce type safety and tight integration with IDEs. Bot&#39;s commands <a href="https://github.com/teloxide/teloxide#commands">have precise types too</a>, thereby serving as a self-documenting code and respecting the <a href="https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/">parse, don&#39;t validate</a> programming idiom.
</p>

<hr>

<h3 align="center">Persistency</h3>
<p align="center">
By default, teloxide stores all user dialogues in RAM, but you can store them somewhere else (for example, in a database) just by implementing <a href="https://docs.rs/teloxide/latest/teloxide/dispatching/dialogue/trait.Storage.html">2 functions</a>. Default database implementations <a href="https://github.com/teloxide/teloxide/issues/183">are coming</a>!.
</p>

<hr>

<h3 align="center">Convenient dialogues management</h3>
<p align="center">
Define a type-safe <a href="https://en.wikipedia.org/wiki/Finite-state_machine">finite automaton</a> and transition functions to drive a user dialogue with ease (see <a href="#guess-a-number">the guess-a-number example</a> below).
</p>

## Getting started
 1. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 2. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
# Unix
$ export TELOXIDE_TOKEN=<Your token here>

# Windows
$ set TELOXIDE_TOKEN=<Your token here>
```
 3. Be sure that you are up to date:
```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```

 4. Execute `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:
```toml
[dependencies]
teloxide = "0.2.0"
log = "0.4.8"
tokio = "0.2.11"
pretty_env_logger = "0.4.0"
```

## The ping-pong bot
This bot has a single message handler, which answers "pong" to each incoming message:

([Full](https://github.com/teloxide/teloxide/blob/master/examples/ping_pong_bot/src/main.rs))
```rust
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                req!(message.answer("pong")).log_on_error().await;
            })
        })
        .dispatch()
        .await;
}

```

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/PING_PONG_BOT.png width="600" />
  </kbd>
</div>

## Commands
Commands are defined similar to how we define CLI using [structopt](https://docs.rs/structopt/0.3.9/structopt/). This bot says "I am a cat! Meow!" on `/meow`, generates a random number within [0; 1) on `/generate`, and shows the usage guide on `/help`:

([Full](https://github.com/teloxide/teloxide/blob/master/examples/simple_commands_bot/src/main.rs))
```rust
// Imports are omitted...

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
    cx: DispatcherHandlerCx<Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => req!(cx.answer(Command::descriptions()))?,
        Command::Generate => req!(cx.answer(generate()))?,
        Command::Meow => req!(cx.answer("I am a cat! Meow!"))?,
    };

    Ok(())
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    // Only iterate through commands in a proper format:
    rx.commands::<Command, &str>(panic!("Insert here your bot's name"))
        // Execute all incoming commands concurrently:
        .for_each_concurrent(None, |(cx, command, _)| async move {
            answer(cx, command).await.log_on_error().await;
        })
        .await;
}

#[tokio::main]
async fn main() {
    // Setup is omitted...
}
```

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/SIMPLE_COMMANDS_BOT.png width="500"/>
  </kbd>
  <br/><br/>
</div>

See? The dispatcher gives us a stream of messages, so we can handle it as we want! Here we use our `.commands::<Command>()` and [`.for_each_concurrent()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.for_each_concurrent), but others are also available:
 - [`.filter()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.filter) / [`.filter_map()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.filter_map) to filter certain kinds of updates;
 - [`.inspect()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.inspect) for debugging purposes;
 - [`.for_each_concurrent()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.for_each_concurrent) + [`tokio::sync::watch`](https://docs.rs/tokio/0.2.13/tokio/sync/watch/index.html) to register multiple handlers;
 - [`.text_messages()`](https://docs.rs/teloxide/0.2.0/teloxide/dispatching/trait.DispatcherHandlerRxExt.html#tymethod.text_messages) to receive only text messages;
 
 - ... And lots of [others](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html) and [others](https://docs.rs/teloxide/latest/teloxide/dispatching/trait.DispatcherHandlerRxExt.html) and [others](https://docs.rs/tokio/0.2.13/tokio/sync/index.html)!

## Guess a number
Wanna see more? This is a bot, which starts a game on each incoming message. You must guess a number from 1 to 10 (inclusively):

([Full](https://github.com/teloxide/teloxide/blob/master/examples/guess_a_number_bot/src/main.rs))
```rust
// Imports are omitted...

type Secret = u8;

#[derive(SmartDefault)]
enum Dialogue {
    #[default]
    Start,
    ReceiveAttempt(Secret),
}

type Cx<State> = DialogueDispatcherHandlerCx<Message, State, Infallible>;
type Res = ResponseResult<DialogueStage<Dialogue>>;

async fn start(cx: Cx<()>) -> Res {
    req!(cx.answer(
        "Let's play a game! Guess a number from 1 to 10 (inclusively)."
    ))?;
    next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
}

async fn receive_attempt(cx: Cx<Secret>) -> Res {
    let secret = cx.dialogue.unwrap();

    match cx.update.text() {
        None => {
            req!(cx.answer("Oh, please, send me a text message!"))?;
            next(Dialogue::ReceiveAttempt(secret))
        }
        Some(text) => match text.parse::<Secret>() {
            Ok(attempt) => {
                if attempt == secret {
                    req!(cx.answer("Congratulations! You won!"))?;
                    exit()
                } else {
                    req!(cx.answer("No."))?;
                    next(Dialogue::ReceiveAttempt(secret))
                }
            }
            Err(_) => {
                req!(cx.answer(
                    "Oh, please, send me a number in the range [1; 10]!"
                ))?;
                next(Dialogue::ReceiveAttempt(secret))
            }
        },
    }
}

async fn handle_message(
    cx: DialogueDispatcherHandlerCx<Message, Dialogue>,
) -> Res {
    // Match is omitted...
}

#[tokio::main]
async fn main() {
    // Setup is omitted...
}
```

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/GUESS_A_NUMBER_BOT.png width="600" />
  </kbd>
  <br/><br/>
</div>

Our [finite automaton](https://en.wikipedia.org/wiki/Finite-state_machine), designating a user dialogue, cannot be in an invalid state, and this is why it is called "type-safe". We could use `enum` + `Option`s instead, but it would lead us to lots of unpleasant `.unwrap()`s.

Remember that a classical [finite automaton](https://en.wikipedia.org/wiki/Finite-state_machine) is defined by its initial state, a list of its possible states and a transition function? We can think that `Dialogue` is a finite automaton with a context type at each state (`Dialogue::Start` has `()`, `Dialogue::ReceiveAttempt` has `u8`).

See [examples/dialogue_bot](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/main.rs) to see a bit more complicated bot with dialogues.

## [More examples!](https://github.com/teloxide/teloxide/tree/master/examples)

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
 
The second one produces very strange compiler messages because of the `#[tokio::main]` macro. However, the examples in this README use the second variant for brevity.

## FAQ
### Where I can ask questions?
[Issues](https://github.com/teloxide/teloxide/issues) is a good place for well-formed questions, for example, about the library design, enhancements, bug reports. But if you can't compile your bot due to compilation errors and need quick help, feel free to ask in [our official group](https://t.me/teloxide).

### Why Rust?
Most programming languages have their own implementations of Telegram bots frameworks, so why not Rust? We think Rust provides enough good ecosystem and the language itself to be suitable for writing bots.

### Can I use webhooks?
teloxide doesn't provide special API for working with webhooks due to their nature with lots of subtle settings. Instead, you setup your webhook by yourself, as shown in [webhook_ping_pong_bot](examples/webhook_ping_pong_bot/src/main.rs).

Associated links:
 - [Marvin's Marvellous Guide to All Things Webhook](https://core.telegram.org/bots/webhooks)
 - [Using self-signed certificates](https://core.telegram.org/bots/self-signed)

### Can I use different loggers?
Of course, you can. The [`enable_logging!`](https://docs.rs/teloxide/latest/teloxide/macro.enable_logging.html) and [`enable_logging_with_filter!`](https://docs.rs/teloxide/latest/teloxide/macro.enable_logging_with_filter.html) macros are just convenient utilities, not necessary to use them. You can setup a different logger, for example, [fern](https://crates.io/crates/fern), as usual, e.g. teloxide has no specific requirements as it depends only on [log](https://crates.io/crates/log).

## Community bots
Feel free to push your own bot into our collection: https://github.com/teloxide/community-bots. Later you will be able to play with them right in our official chat: https://t.me/teloxide.

## Contributing
See [CONRIBUTING.md](https://github.com/teloxide/teloxide/blob/master/CONTRIBUTING.md).
