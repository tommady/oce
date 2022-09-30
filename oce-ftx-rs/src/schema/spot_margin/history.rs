// 
// Generated by oce-gen
// DO NOT EDIT
// 
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnNull};

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct History {
    // no description
    // example value: USD
    #[serde(rename = "coin")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#coin: String,
    
    // no description
    // example value: 2021-04-06T20:00:00+00:00
    #[serde(rename = "time")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#time: chrono::DateTime<chrono::Utc>,
    
    // hourly lending rate for this cycle
    // example value: 0.00002283
    #[serde(rename = "rate")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#rate: rust_decimal::Decimal,
    
    // total size matched between borrowers and lenders
    // example value: 615974048.2224404
    #[serde(rename = "size")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#size: rust_decimal::Decimal,
    }