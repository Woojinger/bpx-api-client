use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::order::{LimitOrder, MarketOrder};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchAccountPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoBorrowSettlements: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoLend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoRepayBorrows: Option<bool>,
    pub leverageLimit: Option<Decimal>,
}

