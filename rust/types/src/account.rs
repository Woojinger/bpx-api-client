use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::order::{LimitOrder, MarketOrder};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchAccountPayload {
    pub autoBorrowSettlements: bool,
    pub autoLend: bool,
    pub autoRepayBorrows: bool,
    pub leverageLimit: Decimal,
}

