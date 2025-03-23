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
    pub async fn patch_account(&self, paylaod: PatchAccountPayload) -> Result<AccountDetails, Box<dyn Error>> {
        let endpoint = format!("{}{}", self.base_url, API_ACCOUNT);
        let res = self.patch(endpoint, paylaod).await?;
        // Check if the response is successful and not empty
        if res.status().is_success() {
            if res.status().as_u16() == 204 {
                // Handle 204 No Content specifically
                Err("No Content to update".into())
            } else {
                match res.json::<AccountDetails>().await {
                    Ok(account_details) => Ok(account_details),
                    Err(e) => Err(e.into()),
                }
            }
        } else {
            // You can handle different status codes here if needed
            Err(format!("Failed with status: {}", res.status()).into())
        }
    }
}
