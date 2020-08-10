use crate::{
    net,
    requests::{form_builder::FormBuilder, RequestWithFile, ResponseResult},
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
    Bot,
};

/// Use this method to send audio files, if you want Telegram clients to 
/// display them in the music player.
///
/// Your audio must be in the .MP3 or .M4A format. Bots can currently send
/// audio files of up to 50 MB in size, this limit may be changed in the
/// future.
///
/// For sending voice messages, use the [`Bot::send_voice`] method instead.
///
/// [The official docs](https://core.telegram.org/bots/api#sendaudio).
///
/// [`Bot::send_voice`]: crate::Bot::send_voice
#[derive(Debug, Clone)]
pub struct SendAudio {
    bot: Bot,
    chat_id: ChatId,
    audio: InputFile,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    duration: Option<i32>,
    performer: Option<String>,
    title: Option<String>,
    thumb: Option<InputFile>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl RequestWithFile for SendAudio {
    type Output = Message;

    async fn send(&self) -> tokio::io::Result<ResponseResult<Message>> {
        let mut builder = FormBuilder::new()
            .add_text("chat_id", &self.chat_id)
            .add_input_file("audio", &self.audio)
            .await?
            .add_text("caption", &self.caption)
            .add_text("parse_mode", &self.parse_mode)
            .add_text("duration", &self.duration)
            .add_text("performer", &self.performer)
            .add_text("title", &self.title)
            .add_text("disable_notification", &self.disable_notification)
            .add_text("reply_to_message_id", &self.reply_to_message_id)
            .add_text("reply_markup", &self.reply_markup);
        if let Some(thumb) = self.thumb.as_ref() {
            builder = builder.add_input_file("thumb", thumb).await?;
        }
        Ok(net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendAudio",
            builder.build(),
        )
        .await)
    }
}

impl SendAudio {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, audio: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            audio,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
            thumb: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Audio file to send.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub fn audio(mut self, val: InputFile) -> Self {
        self.audio = val;
        self
    }

    /// Audio caption, 0-1024 characters.
    pub fn caption<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    /// Send [Markdown] or [HTML], if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    /// Duration of the audio in seconds.
    pub fn duration(mut self, val: i32) -> Self {
        self.duration = Some(val);
        self
    }

    /// Performer.
    pub fn performer<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.performer = Some(val.into());
        self
    }

    /// Track name.
    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(val.into());
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side.
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size. A
    /// thumbnail‘s width and height should not exceed 320. Ignored if the
    /// file is not uploaded using `multipart/form-data`. Thumbnails can’t
    /// be reused and can be only uploaded as a new file, so you can pass
    /// `attach://<file_attach_name>` if the thumbnail was uploaded using
    /// `multipart/form-data` under `<file_attach_name>`. [More info on
    /// Sending Files »].
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    /// If the message is a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    /// Additional interface options. A JSON-serialized object for an [inline
    /// keyboard], [custom reply keyboard], instructions to remove reply
    /// keyboard or to force a reply from the user.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
