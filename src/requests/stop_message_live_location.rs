use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::{InlineKeyboardMarkup, Message},
};
use std::borrow::Cow;

/// Use this method to stop updating a live location message before live_period
/// expires. On success, if the message was sent by the bot, the sent Message is
/// returned, otherwise True is returned.
#[derive(Debug, Clone, Serialize)]
pub struct StopMessageLiveLocation<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat or username of the target channel (in the format
    /// @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Cow<'a, ChatId>>,
    /// Required if inline_message_id is not specified. Identifier of the
    /// message with live location to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Cow<'a, str>>,
    /// A JSON-serialized object InlineKeyboardMarkup for a new inline
    /// keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Cow<'a, InlineKeyboardMarkup>>,
}

#[async_trait]
impl Request for StopMessageLiveLocation<'_> {
    type ReturnValue = Message;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl StopMessageLiveLocation<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "stopMessageLiveLocation",
            &self,
        )
        .await
    }
}

impl<'a> StopMessageLiveLocation<'a> {
    pub(crate) fn new(ctx: RequestContext<'a>) -> Self {
        Self {
            ctx,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<Cow<'a, ChatId>>,
    {
        self.chat_id = Some(chat_id.into());
        self
    }

    pub fn message_id<T>(mut self, message_id: T) -> Self
    where
        T: Into<i32>,
    {
        self.message_id = Some(message_id.into());
        self
    }

    pub fn inline_message_id<T>(mut self, inline_message_id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.inline_message_id = Some(inline_message_id.into());
        self
    }

    pub fn reply_markup<T>(mut self, reply_markup: T) -> Self
    where
        T: Into<Cow<'a, InlineKeyboardMarkup>>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}
