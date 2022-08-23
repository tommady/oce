use crate::schema;

use reqwest::Method;

pub struct GetMarkets {}

impl GetMarkets {
    pub fn new() -> GetMarkets {
        GetMarkets {}
    }
}

impl crate::rest::Request for GetMarkets {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets";
    const IS_AUTH: bool = false;
    type Response = Vec<schema::market::Market>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::RestBuilder;
    use anyhow::Result;

    #[tokio::test]
    async fn test_get_markets() -> Result<()> {
        let rest = RestBuilder::new().build()?;
        let resp = rest.call(GetMarkets::new()).await?;

        assert!(resp.success);
        assert_ne!(
            resp.result.len(),
            0,
            " GetMarkets returns array should not be zero"
        );
        Ok(())
    }
}
