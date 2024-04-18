use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, Sticker, StickerType};

/// This object represents a sticker set.
///
/// [The official docs](https://core.telegram.org/bots/api#stickerset).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name.
    pub name: String,

    /// Sticker set title.
    pub title: String,

    /// Sticker type shared by all stickers in this set.
    #[serde(flatten)]
    pub kind: StickerType,

    /// List of all set stickers.
    pub stickers: Vec<Sticker>,

    /// Sticker set thumbnail in the `.webp`, `.tgs` or `.webm` format.
    pub thumb: Option<PhotoSize>,
}

/// This allows calling [`StickerType`]'s methods directly on [`StickerSet`].
///
/// ```no_run
/// use teloxide_core::types::StickerSet;
///
/// let sticker: StickerSet = todo!();
///
/// let _ = sticker.is_mask();
/// let _ = sticker.kind.is_mask();
/// ```
impl Deref for StickerSet {
    type Target = StickerType;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use crate::types::StickerSet;

    #[test]
    fn smoke_serde() {
        // https://t.me/addstickers/teloxide_test
        let json = r#"{
            "name": "teloxide_test",
            "title": "teloxide-test",
            "sticker_type": "regular",
            "contains_masks": false,
            "stickers": [
                {
                    "width": 512,
                    "height": 512,
                    "emoji": "⚙️",
                    "set_name": "teloxide_test",
                    "is_animated": false,
                    "is_video": false,
                    "type": "regular",
                    "thumbnail": {
                        "file_id": "AAMCAQADFQABYzB4ATH0sqXx351gZ5GpY1Z3Tl8AAlgCAAJ1t4hFbxNCoAg1-akBAAdtAAMpBA",
                        "file_unique_id": "AQADWAIAAnW3iEVy",
                        "file_size": 7698,
                        "width": 320,
                        "height": 320
                    },
                    "thumb": {
                        "file_id": "AAMCAQADFQABYzB4ATH0sqXx351gZ5GpY1Z3Tl8AAlgCAAJ1t4hFbxNCoAg1-akBAAdtAAMpBA",
                        "file_unique_id": "AQADWAIAAnW3iEVy",
                        "file_size": 7698,
                        "width": 320,
                        "height": 320
                    },
                    "file_id": "CAACAgEAAxUAAWMweAEx9LKl8d-dYGeRqWNWd05fAAJYAgACdbeIRW8TQqAINfmpKQQ",
                    "file_unique_id": "AgADWAIAAnW3iEU",
                    "file_size": 12266
                },
                {
                    "width": 512,
                    "height": 512,
                    "emoji": "⚙️",
                    "set_name": "teloxide_test",
                    "is_animated": false,
                    "is_video": false,
                    "type": "regular",
                    "thumbnail": {
                        "file_id": "AAMCAQADFQABYzB4AcABR8-MuvGagis9Pk6liSAAAs8DAAL2YYBFNbvduoN1p7oBAAdtAAMpBA",
                        "file_unique_id": "AQADzwMAAvZhgEVy",
                        "file_size": 7780,
                        "width": 320,
                        "height": 320
                    },
                    "thumb": {
                        "file_id": "AAMCAQADFQABYzB4AcABR8-MuvGagis9Pk6liSAAAs8DAAL2YYBFNbvduoN1p7oBAAdtAAMpBA",
                        "file_unique_id": "AQADzwMAAvZhgEVy",
                        "file_size": 7780,
                        "width": 320,
                        "height": 320
                    },
                    "file_id": "CAACAgEAAxUAAWMweAHAAUfPjLrxmoIrPT5OpYkgAALPAwAC9mGARTW73bqDdae6KQQ",
                    "file_unique_id": "AgADzwMAAvZhgEU",
                    "file_size": 12158
                }
            ]
        }"#;

        let set: StickerSet = serde_json::from_str(json).unwrap();

        assert!(set.is_regular());
        assert!(set.thumb.is_none());
        assert_eq!(set.stickers.len(), 2);
    }
}
