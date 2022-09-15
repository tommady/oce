pub mod market;
pub mod orderbook;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub result: T,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
