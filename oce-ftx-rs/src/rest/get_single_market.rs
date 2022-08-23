use crate::schema;

use reqwest::Method;

use std::borrow::Cow;

pub struct GetSingleMarket<'a> {
    market_name: &'a str,
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
