use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Market {
    // e.g. "BTC/USD" for spot, "BTC-PERP" for futures
    // example value: BTC-PERP
    #[serde(rename = "name")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#name: String,

    // spot markets only
    // example value: BTC
    #[serde(rename = "baseCurrency")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#base_currency: Option<String>,

    // spot markets only
    // example value: USD
    #[serde(rename = "quoteCurrency")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#quote_currency: Option<String>,

    // no description
    // example value: 28914.76
    #[serde(rename = "quoteVolume24h")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#quote_volume24h: Decimal,

    // change in the past hour
    // example value: 0.012
    #[serde(rename = "change1h")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#change1h: Decimal,

    // change in the past 24 hours
    // example value: 0.0299
    #[serde(rename = "change24h")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#change24h: Decimal,

    // change since start of day (00:00 UTC)
    // example value: 0.0156
    #[serde(rename = "changeBod")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#change_bod: Decimal,

    // no description
    // example value: false
    #[serde(rename = "highLeverageFeeExempt")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#high_leverage_fee_exempt: bool,

    // Minimum maker order size (if >10 orders per hour fall below this size)
    // example value: 0.001
    #[serde(rename = "minProvideSize")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#min_provide_size: Decimal,

    // "future" or "spot"
    // example value: future
    #[serde(rename = "type")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#type: String,

    // future markets only
    // example value: BTC
    #[serde(rename = "underlying")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#underlying: Option<String>,

    // no description
    // example value: true
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#enabled: bool,

    // best ask
    // example value: 3949.25
    #[serde(rename = "ask")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#ask: Decimal,

    // best bid
    // example value: 3949
    #[serde(rename = "bid")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#bid: Decimal,

    // last traded price
    // example value: 3949
    #[serde(rename = "last")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#last: Decimal,

    // if the market is in post-only mode (all orders get modified to be post-only, in addition to other settings they may have)
    // example value: false
    #[serde(rename = "postOnly")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#post_only: bool,

    // current price
    // example value: 10579.52
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#price: Decimal,

    // no description
    // example value: 0.25
    #[serde(rename = "priceIncrement")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#price_increment: Decimal,

    // no description
    // example value: 0.0001
    #[serde(rename = "sizeIncrement")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#size_increment: Decimal,

    // if the market has nonstandard restrictions on which jurisdictions can trade it
    // example value: false
    #[serde(rename = "restricted")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#restricted: bool,

    // USD volume in past 24 hours
    // example value: 28914.76
    #[serde(rename = "volumeUsd24h")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#volume_usd24h: Decimal,

    // threshold above which an order is considered large (for VIP rate limits)
    // example value: 5000
    #[serde(rename = "largeOrderThreshold")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#large_order_threshold: Decimal,

    // if the market has an ETF as its baseCurrency
    // example value: false
    #[serde(rename = "isEtfMarket")]
    #[serde(deserialize_with = "crate::schema::deserialize_null_default")]
    r#is_etf_market: bool,
}
