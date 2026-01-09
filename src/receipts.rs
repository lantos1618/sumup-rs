use crate::{Receipt, Result, SumUpClient};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ReceiptRetrieveQuery {
    pub mid: String,
}

impl SumUpClient {
    /// Retrieves a receipt by ID.
    pub async fn retrieve_receipt(&self, receipt_id: &str, query: &ReceiptRetrieveQuery) -> Result<Receipt> {
        let url = self.build_url(&format!("/v1.1/receipts/{}", receipt_id))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).query(query).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves a receipt for a specific merchant.
    pub async fn retrieve_merchant_receipt(&self, merchant_code: &str, receipt_id: &str, query: &ReceiptRetrieveQuery) -> Result<Receipt> {
        let url = self.build_url(&format!("/v1.1/merchants/{}/receipts/{}", merchant_code, receipt_id))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).query(query).send().await?;
        self.handle_response(response).await
    }
}
