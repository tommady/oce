// 
// Generated by oce-gen
// DO NOT EDIT
// 
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnNull};

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Orderbook {
    // Array with price and size
    // example value: [[4114.25,6.263]]
    #[serde(rename = "asks")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#asks: Vec<Vec<rust_decimal::Decimal>>,
    
    // Array with price and size
    // example value: [[4112,49.29]]
    #[serde(rename = "bids")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#bids: Vec<Vec<rust_decimal::Decimal>>,
    }