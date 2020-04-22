// This bot answers how many messages it received in total on every message.

use std::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use teloxide::prelude::*;

lazy_static! {
    static ref MESSAGES_TOTAL: AtomicU64 = AtomicU64::new(0);
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting shared_state_bot!");

    let bot = Bot::from_env();

    polling_default(bot)
        .basic_config()
        .for_each_concurrent(None, |message| async move {
            let previous = MESSAGES_TOTAL.fetch_add(1, Ordering::Relaxed);

            message
                .answer(format!("I received {} messages in total.", previous))
                .send()
                .await
                .log_on_error()
                .await;
        })
        .await;
}
