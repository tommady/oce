mod get_markets;
mod get_orderbook;
mod get_single_market;
mod get_trades;

use reqwest::{Client, ClientBuilder, Method};
use serde::de::DeserializeOwned;

use crate::{
    errors::FtxError,
    schema::{ErrorResponse, SuccessResponse},
    Result,
};

use std::{borrow::Cow, time::Duration};

pub trait Request {
    const METHOD: Method;
    const PATH: &'static str;
    const IS_AUTH: bool;

    type Response: DeserializeOwned;

    fn path(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::PATH)
    }
}

pub struct RestBuilder<'a> {
    token: &'a str,
    timeout_sec: u64,
}

impl<'a> Default for RestBuilder<'a> {
    fn default() -> RestBuilder<'a> {
        RestBuilder::new()
    }
}

impl<'a> RestBuilder<'a> {
    pub fn new() -> RestBuilder<'a> {
        RestBuilder {
            token: "",
            timeout_sec: 3,
        }
    }

    pub fn token(mut self, token: &'a str) -> RestBuilder {
        self.token = token;
        self
    }

    pub fn timeout_sec(mut self, sec: u64) -> RestBuilder<'a> {
        self.timeout_sec = sec;
        self
    }

    pub fn build(&self) -> Result<Rest<'a>> {
        Ok(Rest {
            token: self.token,
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(self.timeout_sec))
                .build()?,
        })
    }
}

const FTX_API_URL: &str = "https://ftx.com/api";

pub struct Rest<'a> {
    token: &'a str,
    client: Client,
}

impl<'a> Rest<'a> {
    pub async fn call<R>(&self, request: R) -> Result<SuccessResponse<R::Response>>
    where
        R: Request,
    {
        let req = self.client.get(&[FTX_API_URL, &request.path()].join(""));

        match req.send().await {
            Ok(res) => {
                if res.status() != 200 {
                    let err: ErrorResponse = res.json().await?;
                    return Err(FtxError::Ftx {
                        success: err.success,
                        error: err.error,
                    });
                }
                Ok(res.json().await?)
            }
            Err(e) => Err(FtxError::Reqwest(e)),
        }
    }
}
