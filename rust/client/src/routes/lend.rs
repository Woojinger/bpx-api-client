use std::error::Error;
use tracing::info;
use bpx_api_types::lend::{BorrowLendPosition, RequestBorrowLendPayload, RequestBorrowLendResponse};


use crate::error::Result;
use crate::BpxClient;

pub const API_BORROW_LEND_POSITION: &str = "/api/v1/borrowLend/positions";
pub const API_BORROW_LEND: &str = "/api/v1/borrowLend";

impl BpxClient {
    pub async fn get_borrow_lend_positions(&self) -> Result<Vec<BorrowLendPosition>> {
        let mut url = format!("{}{}", self.base_url, API_BORROW_LEND_POSITION);
        let res = self.get(&url).await?;
        res.json().await.map_err(Into::into)
    }
    
    pub async fn execute_borrow_lend(&self, payload: RequestBorrowLendPayload) -> Result<(RequestBorrowLendResponse)> {
        let endpoint = format!("{}{}", self.base_url, API_BORROW_LEND);
        let res = self.post(endpoint, payload).await?;
        // Check if the response is successful and not empty
        if res.content_length().unwrap_or(0) == 0 || res.status().is_success() {
            Ok((RequestBorrowLendResponse{
                code: "".to_string(),
                message: "".to_string(),
            }))
        } else {
            res.json().await.map_err(Into::into)
        }
    }
}