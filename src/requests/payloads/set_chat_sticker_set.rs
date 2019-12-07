use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, True},
};

/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    chat_id: ChatId,
    /// Name of the sticker set to be set as the group sticker set
    sticker_set_name: String,
}

impl Method for SetChatStickerSet {
    type Output = True;

    const NAME: &'static str = "setChatStickerSet";
}

impl json::Payload for SetChatStickerSet {}

impl dynamic::Payload for SetChatStickerSet {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SetChatStickerSet {
    pub fn new<C, S>(chat_id: C, sticker_set_name: S) -> Self
    where
        C: Into<ChatId>,
        S: Into<String>
    {
        let chat_id = chat_id.into();
        let sticker_set_name = sticker_set_name.into();
        Self {
            chat_id,
            sticker_set_name,
        }
    }
}

impl json::Request<'_, SetChatStickerSet> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn sticker_set_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.sticker_set_name = val.into();
        self
    }
}
                 