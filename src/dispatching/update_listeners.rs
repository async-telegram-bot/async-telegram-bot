//! Receiving updates from Telegram.
//!
//! The key trait here is [`UpdateListener`]. You can get it by these functions:
//!
//!  - [`polling_default`], which returns a default long polling listener.
//!  - [`polling`], which returns a long/short polling listener with your
//!    configuration.
//!
//! And then you can extract updates from it and pass them directly to a
//! dispatcher.
//!
//! Telegram supports two ways of [getting updates]: [long]/[short] polling and
//! [webhook].
//!
//! # Long Polling
//!
//! In long polling, you just call [`Box::get_updates`] every N seconds.
//!
//! ## Example
//!
//! <pre>
//!     tg                           bot
//!      |                            |
//!      |<---------------------------| Updates? (Bot::get_updates call)
//!      ↑                            ↑
//!      |          timeout<a id="1b" href="#1">^1</a>         |
//!      ↓                            ↓
//! Nope |--------------------------->|
//!      ↑                            ↑
//!      | delay between Bot::get_updates<a id="2b" href="#2">^2</a> |
//!      ↓                            ↓
//!      |<---------------------------| Updates?
//!      ↑                            ↑
//!      |          timeout<a id="3b" href="#3">^3</a>         |
//!      ↓                            ↓
//! Yes  |-------[updates 0, 1]------>|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 1]--------| Updates?<a id="4b" href="#4">^4</a>
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Yes  |---------[update 2]-------->|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 2]--------| Updates?
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Nope |--------------------------->|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 2]--------| Updates?
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Nope |--------------------------->|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 2]--------| Updates?
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Yes  |-------[updates 2..5]------>|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 5]--------| Updates?
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Nope |--------------------------->|
//!      |                            |
//!      ~    and so on, and so on    ~
//! </pre>
//!
//! <a id="1" href="#1b">^1</a> A timeout can be even 0
//!   (this is also called short polling),
//!   but you should use it **only** for testing purposes.
//!
//! <a id="2" href="#2b">^2</a> Large delays will cause in bot lags,
//!   so delay shouldn't exceed second.
//!
//! <a id="3" href="#3b">^3</a> Note that if Telegram already have updates for
//!   you it will answer you **without** waiting for a timeout.
//!
//! <a id="4" href="#4b">^4</a> `offset = N` means that we've already received
//!   updates `0..=N`.
//!
//! [`UpdateListener`]: UpdateListener
//! [`polling_default`]: polling_default
//! [`polling`]: polling
//! [`Box::get_updates`]: crate::Bot::get_updates
//! [getting updates]: https://core.telegram.org/bots/api#getting-updates
//! [long]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
//! [short]: https://en.wikipedia.org/wiki/Polling_(computer_science)
//! [webhook]: https://en.wikipedia.org/wiki/Webhook

use futures::{stream, Stream, StreamExt};

use crate::{
    bot::Bot,
    requests::Request,
    types::{AllowedUpdate, Update},
    RequestError,
};
use std::{convert::TryInto, time::Duration};

/// A generic update listener.
pub trait UpdateListener<E>: Stream<Item = Result<Update, E>> {
    // TODO: add some methods here (.shutdown(), etc).
}
impl<S, E> UpdateListener<E> for S where S: Stream<Item = Result<Update, E>> {}

pub struct Polling<'a> {
    running: bool,
    marker: std::marker::PhantomData<&'a ()>
}

impl<'a> Polling<'a> {
    pub fn new() -> Self {
        Self {
            running: true,
            marker: std::marker::PhantomData::<&'a ()>
        }
    }

    pub fn update_listener(
        &'a self,
        bot: &'a Bot,
        timeout: Option<Duration>,
        limit: Option<u8>,
        allowed_updates: Option<Vec<AllowedUpdate>>,
    ) -> impl UpdateListener<RequestError> + 'a {
        let timeout =
            timeout.map(|t| t.as_secs().try_into().expect("timeout is too big"));

        stream::unfold(
            (allowed_updates, bot, 0, &self.running),
            move |(mut allowed_updates, bot, mut offset, running)| async move {
                if !running {
                    return None;
                }
                let mut req = bot.get_updates().offset(offset);
                req.timeout = timeout;
                req.limit = limit;
                req.allowed_updates = allowed_updates.take();

                let updates = match req.send().await {
                    Err(err) => vec![Err(err)],
                    Ok(updates) => {
                        if let Some(upd) = updates.last() {
                            offset = upd.id + 1;
                        }
                        updates.into_iter().map(Ok).collect::<Vec<_>>()
                    }
                };

                Some((stream::iter(updates), (allowed_updates, bot, offset, running)))
            },
        )
        .flatten()
    }

    pub fn shutdown(&mut self) {
        self.running = false;
    }
}
/// Returns a long polling update listener with the default configuration.
///
/// See also: [`polling`](polling).
pub fn polling_default(bot: &Bot) -> impl UpdateListener<RequestError> + '_ {
    polling(bot, None, None, None)
}

/// Returns a long/short polling update listener with some additional options.
///
/// - `bot`: Using this bot, the returned update listener will receive updates.
/// - `timeout`: A timeout for polling.
/// - `limit`: Limits the number of updates to be retrieved at once. Values
///   between 1—100 are accepted.
/// - `allowed_updates`: A list the types of updates you want to receive.
/// See [`GetUpdates`] for defaults.
///
/// See also: [`polling_default`](polling_default).
///
/// [`GetUpdates`]: crate::requests::GetUpdates
pub fn polling(
    bot: &Bot,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
) -> impl UpdateListener<RequestError> + '_ {
    let timeout =
        timeout.map(|t| t.as_secs().try_into().expect("timeout is too big"));

    stream::unfold(
        (allowed_updates, bot, 0),
        move |(mut allowed_updates, bot, mut offset)| async move {
            let mut req = bot.get_updates().offset(offset);
            req.timeout = timeout;
            req.limit = limit;
            req.allowed_updates = allowed_updates.take();

            let updates = match req.send().await {
                Err(err) => vec![Err(err)],
                Ok(updates) => {
                    if let Some(upd) = updates.last() {
                        offset = upd.id + 1;
                    }
                    updates.into_iter().map(Ok).collect::<Vec<_>>()
                }
            };

            Some((stream::iter(updates), (allowed_updates, bot, offset)))
        },
    )
    .flatten()
}

// TODO implement webhook (this actually require webserver and probably we
//   should add cargo feature that adds webhook)
//pub fn webhook<'a>(bot: &'a  cfg: WebhookConfig) -> Updater<impl
// Stream<Item=Result<Update, ???>> + 'a> {}
