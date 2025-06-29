use crate::{crypto::secrets::Token, response_types::BuySell};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum EventTrigger {
    Bbo,
    Trades,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OrderbookEvent {
    Add,
    Modify,
    Delete,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MarketLimit {
    Market,
    Limit,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AssetStatus {
    DepositOnly,
    Disabled,
    Enabled,
    FundingTemporarilyDisabled,
    WithdrawalOnly,
    WorkInProgress,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PairStatus {
    CancelOnly,
    Delisted,
    LimitOnly,
    Maintenance,
    Online,
    PostOnly,
    ReduceOnly,
    WorkInProgress,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct TickerSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub event_trigger: Option<EventTrigger>,
    pub snapshot: Option<bool>,
}

impl TickerSubscription {
    pub fn new(symbol: Vec<String>) -> Self {
        TickerSubscription {
            channel: "ticker".to_string(),
            symbol,
            event_trigger: None,
            snapshot: None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Ticker {
    pub ask: f64,
    #[serde(rename = "ask_qty")]
    pub ask_quantity: f64,
    pub bid: f64,
    #[serde(rename = "bid_qty")]
    pub bid_quantity: f64,
    pub change: f64,
    pub change_pct: f64,
    pub high: f64,
    pub last: f64,
    pub low: f64,
    pub symbol: String,
    pub volume: f64,
    pub vwap: f64,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct BookSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub depth: Option<i32>,
    pub snapshot: Option<bool>,
    /// only needed for L3 subscription
    pub token: Option<Token>,
}

impl BookSubscription {
    pub fn new(symbol: Vec<String>) -> Self {
        BookSubscription {
            channel: "book".to_string(),
            symbol,
            depth: None,
            snapshot: None,
            token: None,
        }
    }

    pub fn new_l3(symbol: Vec<String>, token: Token) -> Self {
        BookSubscription {
            channel: "level3".to_string(),
            symbol,
            depth: None,
            snapshot: None,
            token: Some(token),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum L2 {
    Orderbook(Orderbook),
    Update(OrderbookUpdate),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BidAsk {
    pub price: f64,
    #[serde(rename = "qty")]
    pub quantity: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Orderbook {
    pub symbol: String,
    pub checksum: u32,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderbookUpdate {
    pub symbol: String,
    pub checksum: u32,
    pub timestamp: String,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum L3 {
    Orderbook(L3Orderbook),
    Update(L3OrderbookUpdate),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3Orderbook {
    pub symbol: String,
    pub bids: Vec<L3BidAsk>,
    pub asks: Vec<L3BidAsk>,
    pub checksum: u32,
    pub timestamp: String, // rfc3339
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3OrderbookUpdate {
    pub symbol: String,
    pub bids: Vec<L3BidAskUpdate>,
    pub asks: Vec<L3BidAskUpdate>,
    pub checksum: u32,
    pub timestamp: String, // rfc3339
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct L3BidAsk {
    pub order_id: String,
    pub limit_price: f64,
    #[serde(rename = "order_qty")]
    pub order_quantity: f64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3BidAskUpdate {
    pub event: OrderbookEvent,
    pub order_id: String,
    pub limit_price: f64,
    #[serde(rename = "order_qty")]
    pub order_quantity: f64,
    pub timestamp: String,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct OhlcSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub interval: i32,
    pub snapshot: Option<bool>,
}

impl OhlcSubscription {
    pub fn new(symbols: Vec<String>, interval: i32) -> Self {
        OhlcSubscription {
            channel: "ohlc".to_string(),
            symbol: symbols,
            interval,
            snapshot: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionResponse {
    pub channel: String,
    pub symbol: Option<String>,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TradeSubscriptionResponse {
    pub symbol: Option<String>,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OhlcSubscriptionResponse {
    pub symbol: Option<String>,
    pub snapshot: Option<bool>,
    pub interval: i64,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BookSubscriptionResponse {
    pub symbol: String,
    pub depth: Option<i32>,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TickerSubscriptionResponse {
    pub symbol: String,
    pub event_trigger: Option<EventTrigger>,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Ohlc {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub vwap: f64,
    pub trades: i64,
    pub volume: f64,
    pub interval_begin: String,
    pub interval: i32,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct TradesSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub snapshot: Option<bool>,
}

impl TradesSubscription {
    pub fn new(symbols: Vec<String>) -> Self {
        TradesSubscription {
            channel: "trade".to_string(),
            symbol: symbols,
            snapshot: None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Trade {
    pub symbol: String,
    pub side: BuySell,
    #[serde(rename = "qty")]
    pub quantity: f64,
    pub price: f64,
    #[serde(rename = "ord_type")]
    pub order_type: MarketLimit,
    pub trade_id: i64,
    pub timestamp: String,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct InstrumentsSubscription {
    pub channel: String,
    pub snapshot: Option<bool>,
}

impl InstrumentsSubscription {
    pub fn new(snapshot: bool) -> Self {
        InstrumentsSubscription {
            channel: "instrument".to_string(),
            snapshot: Some(snapshot),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Asset {
    pub id: String,
    pub margin_rate: Option<f64>,
    pub precision: i64,
    pub precision_display: i64,
    pub status: AssetStatus,
    pub borrowable: bool,
    pub collateral_value: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Pair {
    pub base: String,
    pub quote: String,
    pub cost_min: f64,
    pub cost_precision: i64,
    pub has_index: bool,
    pub margin_initial: Option<f64>,
    pub marginable: bool,
    pub position_limit_long: Option<i64>,
    pub position_limit_short: Option<i64>,
    pub price_increment: f64,
    pub price_precision: i64,
    #[serde(rename = "qty_increment")]
    pub quantity_increment: f64,
    #[serde(rename = "qty_min")]
    pub quantity_min: f64,
    #[serde(rename = "qty_precision")]
    pub quantity_precision: i64,
    pub status: PairStatus,
    pub symbol: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Instruments {
    pub assets: Vec<Asset>,
    pub pairs: Vec<Pair>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_asset() {
        let raw = r#"{"id":"XLM","status":"enabled","precision":8,"precision_display":5,"borrowable":true,"collateral_value":0.00,"margin_rate":0.020000}"#;
        let expected = Asset {
            id: "XLM".to_string(),
            margin_rate: Some(0.02),
            precision: 8,
            precision_display: 5,
            status: AssetStatus::Enabled,
            borrowable: true,
            collateral_value: 0.0,
        };

        let deserialized = serde_json::from_str::<Asset>(raw).unwrap();

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn test_deserialize_pair() {
        let raw = r#"{"symbol":"ETH/BTC","base":"ETH","quote":"BTC","status":"online","qty_precision":8,"qty_increment":0.00000001,"price_precision":5,"cost_precision":10,"marginable":true,"has_index":true,"cost_min":0.00002,"margin_initial":0.20,"position_limit_long":1000,"position_limit_short":600,"tick_size":0.00001,"price_increment":0.00001,"qty_min":0.00200000}"#;
        let expected = Pair {
            base: "ETH".to_string(),
            quote: "BTC".to_string(),
            cost_min: 0.00002,
            cost_precision: 10,
            has_index: true,
            margin_initial: Some(0.2),
            marginable: true,
            position_limit_long: Some(1000),
            position_limit_short: Some(600),
            price_increment: 0.00001,
            price_precision: 5,
            quantity_increment: 0.00000001,
            quantity_min: 0.002,
            quantity_precision: 8,
            status: PairStatus::Online,
            symbol: "ETH/BTC".to_string(),
        };

        let deserialized = serde_json::from_str::<Pair>(raw).unwrap();

        assert_eq!(expected, deserialized);
    }
}
