use serde::{Deserialize, Serialize};
use bpx_api_types::account::{PatchAccountPayload};
use std::error::Error;


use crate::BpxClient;

pub const API_ACCOUNT: &str = "/api/v1/account";

#[derive(Debug, Deserialize)]
pub struct AccountDetails {
    // Define the fields according to the JSON structure returned by your API
}


impl BpxClient {
    pub async fn patch_account(&self, paylaod: PatchAccountPayload) -> Result<(), Box<dyn Error>> {
        let endpoint = format!("{}{}", self.base_url, API_ACCOUNT);
        let res = self.patch(endpoint, paylaod).await?;
        // Check if the response is successful and not empty
        if res.content_length().unwrap_or(0) == 0 || res.status().is_success() {
            Ok(())
        } else {
            // You can handle different status codes here if needed
            Err(format!("Failed with status: {}", res.status()).into())
        }
    }
}
