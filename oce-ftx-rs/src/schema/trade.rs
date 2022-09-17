use serde::Deserialize;
use rust_decimal::Decimal;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Trade {
    // trade id
    // example value: 3855995
    #[serde(rename = "id")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    pub r#id: u64,
    
    // if this trade involved a liquidation order
    // example value: false
    #[serde(rename = "liquidation")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    pub r#liquidation: bool,
    
    // no description
    // example value: 3857.75
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    pub r#price: Decimal,
    
    // no description
    // example value: buy
    #[serde(rename = "side")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    pub r#side: String,
    
    // no description
    // example value: 0.111
    #[serde(rename = "size")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    pub r#size: Decimal,
    
    // no description
    // example value: 2019-03-20T18:16:23.397991+00:00
    #[serde(rename = "time")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    pub r#time: String,
    }