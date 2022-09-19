use crate::schema;

use reqwest::Method;

use std::borrow::Cow;

pub struct GetTrades<'a> {
    market_name: &'a str,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl<'a> GetTrades<'a> {
    pub fn new(market_name: &'a str) -> GetTrades<'a> {
        GetTrades {
            market_name,
            start_time: None,
            end_time: None,
        }
    }

    pub fn with_start_time(&self, t: u64) -> GetTrades<'a> {
        GetTrades {
            market_name: self.market_name,
            start_time: Some(t),
            end_time: self.end_time,
        }
    }

    pub fn with_end_time(&self, t: u64) -> GetTrades<'a> {
        GetTrades {
            market_name: self.market_name,
            start_time: self.start_time,
            end_time: Some(t),
        }
    }
}

impl<'a> crate::rest::Request for GetTrades<'a> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets";
    const IS_AUTH: bool = false;
    type Response = Vec<schema::trade::Trade>;

    fn path(&self) -> Cow<'_, str> {
        let params = serde_urlencoded::to_string(&[
            ("start_time", self.start_time),
            ("end_time", self.end_time),
        ])
        .unwrap();
        Cow::Owned([Self::PATH, self.market_name].join("/") + "/trades?" + &params)
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

    #[tokio::test]
    async fn test_get_single_market_with_start_and_end_time() -> Result<()> {
        let rest = RestBuilder::new().build()?;
        let resp = rest
            .call(
                GetTrades::new("BTC/USD")
                    .with_start_time(1559881511)
                    .with_end_time(1559881711),
            )
            .await?;

        assert!(resp.success);
        Ok(())
    }
}
