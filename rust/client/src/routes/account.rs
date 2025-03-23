use serde::{Deserialize, Serialize};
use bpx_api_types::account::{PatchAccountPayload};
use std::error::Error;


use crate::BpxClient;

const API_ACCOUNT: &str = "/api/v1/account";

#[derive(Debug, Deserialize)]
pub struct AccountDetails {
    // Define the fields according to the JSON structure returned by your API
}


impl BpxClient {
    pub async fn patch_account(&self, paylaod: PatchAccountPayload) -> Result<AccountDetails, Box<dyn Error>> {
        let endpoint = format!("{}{}", self.base_url, API_ACCOUNT);
        let res = self.patch(endpoint, paylaod).await?;
        res.json().await.map_err(Into::into)
    }
}
