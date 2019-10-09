use std::borrow::Cow;

use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::{Message, ReplyMarkup},
};

/// Use this method to send a native poll. A native poll can't be sent to a
/// private chat. On success, the sent Message is returned.
#[derive(Debug, Clone, Serialize)]
pub struct SendPoll<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// identifier for the target chat or username of the target channel (in
    /// the format @channelusername). A native poll can't be sent to a private
    /// chat.
    chat_id: ChatId<'a>,
    /// Poll question, 1-255 characters
    question: Cow<'a, str>,
    /// List of answer options, 2-10 strings 1-100 characters each
    options: Cow<'a, [str]>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove
    /// or ForceReply 	Optional 	Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove reply keyboard or to force a reply from the user.
    reply_markup: Option<ReplyMarkup<'a>>,
}

#[async_trait]
impl Request for SendPoll<'_> {
    type ReturnValue = Message;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendPoll<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "sendPoll",
            &self,
        )
        .await
    }
}

impl<'a> SendPoll<'a> {
    pub(crate) fn new<C, S, O>(
        ctx: RequestContext<'a>,
        chat_id: C,
        question: S,
        options: O,
    ) -> Self
    where
        C: Into<ChatId<'a>>,
        S: Into<Cow<'a, str>>,
        O: Into<Cow<'a, [str]>>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            question: question.into(),
            options: options.into(),
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId<'a>>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn question<T>(mut self, question: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.question = question.into();
        self
    }

    pub fn options<T>(mut self, options: T) -> Self
    where
        T: Into<Cow<'a, [str]>>,
    {
        self.options = options.into();
        self
    }

    pub fn disable_notification<T>(mut self, disable_notification: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, reply_to_message_id: T) -> Self
    where
        T: Into<i32>,
    {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }

    pub fn reply_markup<T>(mut self, reply_markup: T) -> Self
    where
        T: Into<ReplyMarkup<'a>>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}
