use crate::types::{
    InputFile, InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaPhoto,
    InputMediaVideo,
};
use crate::{
    net,
    requests::{form_builder::FormBuilder, RequestWithFile, ResponseResult},
    types::{ChatOrInlineMessage, InlineKeyboardMarkup, InputMedia, Message},
    Bot,
};

/// Use this method to edit animation, audio, document, photo, or video
/// messages.
///
/// If a message is a part of a message album, then it can be edited only to a
/// photo or a video. Otherwise, message type can be changed arbitrarily. When
/// inline message is edited, new file can't be uploaded. Use previously
/// uploaded file via its `file_id` or specify a URL. On success, if the edited
/// message was sent by the bot, the edited [`Message`] is returned,
/// otherwise [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagemedia).
///
/// [`Message`]: crate::types::Message
/// [`True`]: crate::types::True
#[derive(Debug, Clone)]
pub struct EditMessageMedia {
    bot: Bot,
    chat_or_inline_message: ChatOrInlineMessage,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl RequestWithFile for EditMessageMedia {
    type Output = Message;

    async fn send(&self) -> tokio::io::Result<ResponseResult<Message>> {
        let mut params = FormBuilder::new().add_text("reply_markup", &self.reply_markup);

        match &self.chat_or_inline_message {
            ChatOrInlineMessage::Chat { chat_id, message_id } => {
                let file_id = InputFile::FileId(String::from("attach://media"));
                let input_media = match &self.media {
                    InputMedia::Photo(_media) => InputMedia::Photo(InputMediaPhoto::new(file_id)),
                    InputMedia::Animation(_media) => {
                        InputMedia::Animation(InputMediaAnimation::new(file_id))
                    }
                    InputMedia::Audio(_media) => InputMedia::Audio(InputMediaAudio::new(file_id)),
                    InputMedia::Document(_media) => {
                        InputMedia::Document(InputMediaDocument::new(file_id))
                    }
                    InputMedia::Video(_media) => InputMedia::Video(InputMediaVideo::new(file_id)),
                };
                params = params
                    .add_text("chat_id", chat_id)
                    .add_text("message_id", message_id)
                    .add_text("media", &input_media)
                    .add_input_file("media", &self.media.media())
                    .await?;
            }
            ChatOrInlineMessage::Inline { inline_message_id } => {
                params = params
                    .add_text("inline_message_id", inline_message_id)
                    .add_text("media", &self.media);
            }
        }

        Ok(net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "editMessageMedia",
            params.build(),
        )
        .await)
    }
}

impl EditMessageMedia {
    pub(crate) fn new(
        bot: Bot,
        chat_or_inline_message: ChatOrInlineMessage,
        media: InputMedia,
    ) -> Self {
        Self { bot, chat_or_inline_message, media, reply_markup: None }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    /// A JSON-serialized object for a new media content of the message.
    pub fn media(mut self, val: InputMedia) -> Self {
        self.media = val;
        self
    }

    /// A JSON-serialized object for a new [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
