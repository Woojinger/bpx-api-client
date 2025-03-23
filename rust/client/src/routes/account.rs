const API_ACCOUNT: &str = "/api/v1/account";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchAccountPayload {
    pub autoBorrowSettlements: bool,
    pub autoLend: bool,
    pub autoRepayBorrows: bool,
    pub leverageLimit: Decimal,
}


impl BpxClient {
    pub fn patch_account(&self, paylaod: PatchAccountPayload) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, API_ACCOUNT);
        let res = self.patch(endpoint, paylaod).await?;
        res.json().await.map_err(Into::into)
    }
}
