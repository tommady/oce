// 
// Generated by oce-gen
// DO NOT EDIT
// 
use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Offer {
    // no description
    // example value: BTC
    #[serde(rename = "coin")]
    pub r#coin: String,
    
    // hourly rate at which you will lend, if matched
    // example value: 0.000001
    #[serde(rename = "rate")]
    pub r#rate: rust_decimal::Decimal,
    
    // amount you will lend, if matched
    // example value: 1.9
    #[serde(rename = "size")]
    pub r#size: rust_decimal::Decimal,
    }