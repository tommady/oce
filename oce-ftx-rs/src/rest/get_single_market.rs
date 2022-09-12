use crate::schema;

use reqwest::Method;

use std::borrow::Cow;

pub struct GetSingleMarket<'a> {
    market_name: &'a str,
}

impl<'a> GetSingleMarket<'a> {
    pub fn new(market_name: &'a str) -> GetSingleMarket<'a> {
        GetSingleMarket { market_name }
    }
}

impl<'a> crate::rest::Request for GetSingleMarket<'a> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets";
    const IS_AUTH: bool = false;
    type Response = schema::market::Market;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned([Self::PATH, self.market_name].join("/"))
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
        let resp = rest.call(GetSingleMarket::new("BTC/USD")).await?;

        assert!(resp.success);
        assert_eq!(
            resp.result.name, "BTC/USD",
            " GetMarkets returns array should not be zero"
        );
        Ok(())
    }
}
