use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::Blockchain;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendPosition {
    pub cumulative_interest: Decimal,
    pub id: String,
    pub imf: Decimal,
    pub net_quantity: Decimal,
    pub mark_price: Decimal,
    pub mmf: Decimal,
    pub net_exposure_quantity: Decimal,
    pub net_exposure_notional: Decimal,
    pub symbol: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestBorrowLendPayload {
    pub quantity: Decimal,
    pub side: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestBorrowLendResponse {
    pub code: String,
    pub message: String,
}