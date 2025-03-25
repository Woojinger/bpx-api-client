use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use serde::{Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Option<Decimal>,
    pub executed_quantity: Decimal,
    pub quote_quantity: Option<Decimal>,
    pub executed_quote_quantity: Decimal,
    pub trigger_price: Option<Decimal>,
    pub time_in_force: TimeInForce,
    pub self_trade_prevention: SelfTradePrevention,
    pub status: OrderStatus,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
    pub executed_quantity: Decimal,
    pub executed_quote_quantity: Decimal,
    pub price: Decimal,
    pub trigger_price: Option<Decimal>,
    pub time_in_force: TimeInForce,
    pub self_trade_prevention: SelfTradePrevention,
    pub post_only: bool,
    pub status: OrderStatus,
    pub created_at: i64,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderType {
    #[default]
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "orderType")]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    #[default]
    GTC,
    IOC,
    FOK,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum SelfTradePrevention {
    #[default]
    RejectTaker,
    RejectMaker,
    RejectBoth,
    Allow,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderStatus {
    Cancelled,
    Expired,
    Filled,
    #[default]
    New,
    PartiallyFilled,
    Triggered,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum Side {
    #[default]
    Bid,
    Ask,
}

// Custom serialize function for boolean as string
fn serialize_as_string<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(true) => serializer.serialize_str("true"),
        Some(false) => serializer.serialize_str("false"),
        None => serializer.serialize_none(),
    }
}

// Custom deserialize function for boolean from string
fn deserialize_from_string<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringToBoolVisitor;

    impl<'de> Visitor<'de> for StringToBoolVisitor {
        type Value = Option<bool>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string 'true' or 'false'")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value {
                "true" => Ok(Some(true)),
                "false" => Ok(Some(false)),
                _ => Err(E::custom(format!("unexpected value: {}", value))),
            }
        }
    }

    deserializer.deserialize_option(StringToBoolVisitor)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOrderPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_as_string", deserialize_with = "deserialize_from_string")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention: Option<SelfTradePrevention>,
    pub side: Side,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderPayload {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOpenOrdersPayload {
    pub symbol: String,
}
