use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{stream, Stream, StreamExt};

use crate::{bot::Bot, types::Update, RequestError};
use std::ops::DerefMut;

// Currently just a placeholder, but I'll  add here some methods
/// Updater is stream of updates.
///
/// Telegram supports 2 ways of [getting updates]: [long polling](Long Polling)
/// and webhook
///
/// ## Long Polling
///
/// In long polling ([wiki]) you just call [GetUpdates] every N seconds.
///
/// #### Example:
///
/// <pre>
///     tg                           bot
///      |                            |
///      |<---------------------------| Updates? (GetUpdates call)
///      ↑                            ↑
///      |          timeout<a id="1b" href="#1">^1</a>         |
///      ↓                            ↓
/// Nope |--------------------------->|
///      ↑                            ↑
///      | delay between GetUpdates<a id="2b" href="#2">^2</a> |
///      ↓                            ↓
///      |<---------------------------| Updates?
///      ↑                            ↑
///      |          timeout<a id="3b" href="#3">^3</a>         |
///      ↓                            ↓
/// Yes  |-------[updates 0, 1]------>|
///      ↑                            ↑
///      |           delay            |
///      ↓                            ↓
///      |<-------[offset = 1]--------| Updates?<a id="4b" href="#4">^4</a>
///      ↑                            ↑
///      |           timeout          |
///      ↓                            ↓
/// Yes  |---------[update 2]-------->|
///      ↑                            ↑
///      |           delay            |
///      ↓                            ↓
///      |<-------[offset = 2]--------| Updates?
///      ↑                            ↑
///      |           timeout          |
///      ↓                            ↓
/// Nope |--------------------------->|
///      ↑                            ↑
///      |           delay            |
///      ↓                            ↓
///      |<-------[offset = 2]--------| Updates?
///      ↑                            ↑
///      |           timeout          |
///      ↓                            ↓
/// Nope |--------------------------->|
///      ↑                            ↑
///      |           delay            |
///      ↓                            ↓
///      |<-------[offset = 2]--------| Updates?
///      ↑                            ↑
///      |           timeout          |
///      ↓                            ↓
/// Yes  |-------[updates 2..5]------>|
///      ↑                            ↑
///      |           delay            |
///      ↓                            ↓
///      |<-------[offset = 5]--------| Updates?
///      ↑                            ↑
///      |           timeout          |
///      ↓                            ↓
/// Nope |--------------------------->|
///      |                            |
///      ~    and so on, and so on    ~
/// </pre>
///
/// <a id="1" href="#1b">^1</a> Timeout can be even 0
///   (this is also called short polling),
///   but you should use it **only** for testing purposes
///
/// <a id="2" href="#2b">^2</a> Large delays will cause in bot lags,
///   so delay shouldn't exceed second.
///
/// <a id="3" href="#3b">^3</a> Note that if telegram already have updates for
///   you it will answer you **without** waiting for a timeout
///
/// <a id="4" href="#4b">^4</a> `offset = N` means that we've already received
///   updates `0..=N`
///
/// [GetUpdates]: crate::requests::GetUpdates
/// [getting updates]: https://core.telegram.org/bots/api#getting-updates
/// [wiki]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
pub trait Updater:
    Stream<Item = Result<Update, <Self as Updater>::Error>>
{
    type Error;
}

type StreamItem = Result<Update, RequestError>;

struct InnerUpdater<'a> {
    stream: Box<dyn Stream<Item = StreamItem> + 'a>,
}

impl<'a> InnerUpdater<'a> {
    pub fn new<S>(stream: S) -> Self where S: Stream<Item = StreamItem> + 'a {
        Self { stream: Box::new(stream) }
    }
}

impl Stream for InnerUpdater<'_> {
    type Item = StreamItem;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let i = &mut *self.stream;
        Pin::new(i).poll_next(cx)
    }
}

impl Updater for InnerUpdater<'_> {
    type Error = RequestError;
}

pub fn polling<'a>(bot: &'a Bot) -> InnerUpdater<'a>  {
    let stream = stream::unfold((bot, 0), |(bot, mut offset)| {
        async move {
            // this match converts Result<Vec<_>, _> -> Vec<Result<_, _>>
            let updates = match bot.get_updates().offset(offset).send().await {
                Ok(updates) => {
                    if let Some(upd) = updates.last() {
                        offset = upd.id + 1;
                    }
                    updates.into_iter().map(Ok).collect::<Vec<_>>()
                }
                Err(err) => vec![Err(err)],
            };
            Some((stream::iter(updates), (bot, offset)))
        }
    })
    .flatten();

    InnerUpdater::new(stream)
}

// TODO implement webhook (this actually require webserver and probably we
//   should add cargo feature that adds webhook)
//pub fn webhook<'a>(bot: &'a  cfg: WebhookConfig) -> Updater<impl
// Stream<Item=Result<Update, ???>> + 'a> {}
