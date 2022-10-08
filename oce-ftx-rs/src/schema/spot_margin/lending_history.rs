// 
// Generated by oce-gen
// DO NOT EDIT
// 
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnNull};

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct LendingHistory {
    // no description
    // example value: BTC
    #[serde(rename = "coin")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#coin: String,
    
    // amount of coin you were paid as interest on the loan
    // example value: 0.00047864470072
    #[serde(rename = "proceeds")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#proceeds: rust_decimal::Decimal,
    
    // lending rate
    // example value: 0.00001961096
    #[serde(rename = "rate")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#rate: rust_decimal::Decimal,
    
    // amount lent
    // example value: 24.407
    #[serde(rename = "size")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#size: rust_decimal::Decimal,
    
    // time of interest payment
    // example value: 2020-11-30T12:00:00+00:00
    #[serde(rename = "time")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#time: chrono::DateTime<chrono::Utc>,
    }