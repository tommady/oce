use serde::Deserialize;
use rust_decimal::Decimal;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Orderbook {
    // Array with price and size
    // example value: [4114.25,6.263]
    #[serde(rename = "asks")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    pub r#asks: Vec<Decimal>,
    
    // Array with price and size
    // example value: [4112,49.29]
    #[serde(rename = "bids")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    pub r#bids: Vec<Decimal>,
    }