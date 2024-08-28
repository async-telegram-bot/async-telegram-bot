//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{BusinessConnectionId, ChatAction, Recipient, ThreadId, True};

impl_payload! {
    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
    ///
    /// > Example: The [ImageBot] needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use sendChatAction with action = upload_photo. The user will see a “sending photo” status for the bot.
    ///
    /// We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
    ///
    /// [ImageBot]: https://t.me/imagebot
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendChatAction (SendChatActionSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for [text messages], upload_photo for [photos], record_video or upload_video for [videos], record_audio or upload_audio for [audio files], upload_document for [general files], choose_sticker for [stickers], find_location for [location data], record_video_note or upload_video_note for [video notes].
            ///
            /// [text messages]: crate::payloads::SendMessage
            /// [photos]: crate::payloads::SendPhoto
            /// [videos]: crate::payloads::SendVideo
            /// [audio files]: crate::payloads::SendAudio
            /// [general files]: crate::payloads::SendDocument
            /// [stickers]: crate::payloads::SendSticker
            /// [location data]: crate::payloads::SendLocation
            /// [video notes]: crate::payloads::SendVideoNote
            pub action: ChatAction,
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the action will be sent
            pub business_connection_id: BusinessConnectionId,
            /// Unique identifier for the target message thread; supergroups only
            pub message_thread_id: ThreadId,
        }
    }
}
