use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::Blockchain;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub available: Decimal,
    pub locked: Decimal,
    pub staked: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collateral {
    #[serde(rename = "assetsValue")]
    pub assets_value: Decimal,

    pub borrow_liability: Decimal,
    pub collateral: Vec<CollateralAsset>,
    pub imf: Decimal,
    pub unsettled_equity: Decimal,
    pub liabilities_value: Decimal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_fraction: Option<Decimal>,

    pub mmf: Decimal,

    pub net_equity: Decimal,
    pub net_equity_available: Decimal,
    pub net_equity_locked: Decimal,
    pub net_exposure_futures: Decimal,
    pub pnl_unrealized: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralAsset {
    pub symbol: String,
    pub asset_mark_price: String,
    pub total_quantity: String,
    pub balance_notional: String,
    pub collateral_weight: String,
    pub collateral_value: String,
    pub open_order_quantity: String,
    pub lend_quantity: String,
    pub available_quantity: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    pub id: i32,
    pub to_address: Option<String>,
    pub from_address: Option<String>,
    pub confirmation_block_number: Option<i32>,
    pub identifier: Option<String>,
    pub source: DepositSource,
    pub status: DepositStatus,
    pub subaccount_id: Option<i32>,
    pub symbol: String,
    pub quantity: Decimal,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum DepositSource {
    Administrator,
    Solana,
    Ethereum,
    Bitcoin,
    Nuvei,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum DepositStatus {
    Pending,
    Confirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithdrawalPayload {
    pub address: String,
    pub blockchain: Blockchain,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    pub quantity: Decimal,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    pub id: i32,
    pub blockchain: Blockchain,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    pub quantity: Decimal,
    pub fee: Decimal,
    pub symbol: String,
    pub status: WithdrawalStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<i32>,
    pub to_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum WithdrawalStatus {
    Pending,
    Confirmed,
    Verifying,
    Void,
}
