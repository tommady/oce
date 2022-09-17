use crate::schema;

use reqwest::Method;

use std::borrow::Cow;

pub struct GetTrades<'a> {
    market_name: &'a str,
}

impl<'a> GetTrades<'a> {
    pub fn new(market_name: &'a str) -> GetTrades<'a> {
        GetTrades { market_name }
    }
}

impl<'a> crate::rest::Request for GetTrades<'a> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets";
    const IS_AUTH: bool = false;
    type Response = Vec<schema::trade::Trade>;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned([Self::PATH, self.market_name].join("/") + "/trades")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::RestBuilder;
    use anyhow::Result;

    #[tokio::test]
    async fn test_get_single_market() -> Result<()> {
        let rest = RestBuilder::new().build()?;
        let resp = rest.call(GetTrades::new("BTC/USD")).await?;

        assert!(resp.success);
        assert_ne!(
            resp.result.len(),
            0,
            "GetTrades result length should not be zero"
        );
        Ok(())
    }
}
