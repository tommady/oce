use crate::schema;

use reqwest::Method;

use std::borrow::Cow;

pub struct GetOrderbook<'a> {
    market_name: &'a str,
    depth: Option<usize>,
}

impl<'a> GetOrderbook<'a> {
    pub fn new(market_name: &'a str) -> GetOrderbook<'a> {
        GetOrderbook {
            market_name,
            depth: None,
        }
    }

    pub fn with_depth(&self, depth: usize) -> GetOrderbook<'a> {
        GetOrderbook {
            market_name: self.market_name,
            depth: Some(depth),
        }
    }
}

impl<'a> crate::rest::Request for GetOrderbook<'a> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets";
    const IS_AUTH: bool = false;
    type Response = schema::orderbook::Orderbook;

    fn path(&self) -> Cow<'_, str> {
        match self.depth {
            Some(d) => Cow::Owned(
                [Self::PATH, self.market_name].join("/")
                    + "/orderbook"
                    + "?depth="
                    + &d.to_string(),
            ),
            None => Cow::Owned([Self::PATH, self.market_name].join("/") + "/orderbook"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::RestBuilder;
    use anyhow::Result;

    #[tokio::test]
    async fn test_get_orderbook_depth_1() -> Result<()> {
        let rest = RestBuilder::new().build()?;
        let req = GetOrderbook::new("BTC/USD").with_depth(1);
        let resp = rest.call(req).await?;

        assert!(resp.success);
        assert_eq!(
            resp.result.bids.len(),
            1,
            "GetOrderbook with depth 1 but the bids length is not equal"
        );
        assert_eq!(
            resp.result.asks.len(),
            1,
            "GetOrderbook with depth 1 but the asks length is not equal"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_orderbook_depth_9() -> Result<()> {
        let rest = RestBuilder::new().build()?;
        let req = GetOrderbook::new("BTC/USD").with_depth(9);
        let resp = rest.call(req).await?;

        assert!(resp.success);
        assert_eq!(
            resp.result.bids.len(),
            9,
            "GetOrderbook with depth 9 but the bids length is not equal"
        );
        assert_eq!(
            resp.result.asks.len(),
            9,
            "GetOrderbook with depth 9 but the asks length is not equal"
        );
        Ok(())
    }
}
