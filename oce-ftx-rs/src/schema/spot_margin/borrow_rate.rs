// 
// Generated by oce-gen
// DO NOT EDIT
// 
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnNull};

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct BorrowRate {
    // no description
    // example value: BTC
    #[serde(rename = "coin")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#coin: String,
    
    // estimated hourly borrow rate for the next spot margin cycle
    // example value: 0.00000145
    #[serde(rename = "estimate")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#estimate: rust_decimal::Decimal,
    
    // hourly borrow rate in the previous spot margin cycle
    // example value: 0.00000144
    #[serde(rename = "previous")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub r#previous: rust_decimal::Decimal,
    }