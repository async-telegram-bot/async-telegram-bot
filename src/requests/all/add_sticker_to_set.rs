use crate::{
    net,
    requests::form_builder::FormBuilder,
    types::{InputFile, MaskPosition, True},
    Bot,
};

use crate::requests::{Request, ResponseResult};
use std::sync::Arc;

/// Use this method to add a new sticker to a set created by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#addstickertoset).
#[derive(Debug, Clone)]
pub struct AddStickerToSet {
    bot: Arc<Bot>,
    user_id: i32,
    name: String,
    png_sticker: InputFile,
    tgs_sticker: Option<InputFile>,
    emojis: String,
    mask_position: Option<MaskPosition>,
}

#[async_trait::async_trait]
impl Request for AddStickerToSet {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "addStickerToSet",
            FormBuilder::new()
                .add("user_id", &self.user_id)
                .await
                .add("name", &self.name)
                .await
                .add("png_sticker", &self.png_sticker)
                .await
                .add("tgs_sticker", &self.tgs_sticker)
                .await
                .add("emojis", &self.emojis)
                .await
                .add("mask_position", &self.mask_position)
                .await
                .build(),
        )
        .await
    }
}

impl AddStickerToSet {
    pub(crate) fn new<N, E>(
        bot: Arc<Bot>,
        user_id: i32,
        name: N,
        png_sticker: InputFile,
        emojis: E,
    ) -> Self
    where
        N: Into<String>,
        E: Into<String>,
    {
        Self {
            bot,
            user_id,
            name: name.into(),
            png_sticker,
            emojis: emojis.into(),
            tgs_sticker: None,
            mask_position: None,
        }
    }

    /// User identifier of sticker set owner.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    /// Sticker set name.
    pub fn name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.name = val.into();
        self
    }

    /// **Png** image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    pub fn png_sticker(mut self, val: InputFile) -> Self {
        self.png_sticker = val;
        self
    }
    /// **TGS** animation with the sticker, uploaded using multipart/form-data.
    /// See https://core.telegram.org/animated_stickers#technical-requirements
    /// for technical requirements
    pub fn tgs_sticker(mut self, val: InputFile) -> Self {
        self.tgs_sticker = Some(val);
        self
    }

    /// One or more emoji corresponding to the sticker.
    pub fn emojis<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.emojis = val.into();
        self
    }

    /// A JSON-serialized object for position where the mask should be placed on
    /// faces.
    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }
}
