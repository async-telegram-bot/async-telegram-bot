use crate::types::LabeledPrice;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
/// This object represents one shipping option.
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let shipping_option = ShippingOption {
            id: "0".to_string(),
            title: "Option".to_string(),
            prices: vec![LabeledPrice {
                label: "Label".to_string(),
                amount: 60,
            }],
        };
        let expected = r#"{"id":"0","title":"Option","prices":[{"label":"Label","amount":60}]}"#;
        let actual = serde_json::to_string(&shipping_option).unwrap();
        assert_eq!(actual, expected);
    }
}
