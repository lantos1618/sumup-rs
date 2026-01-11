use crate::{Receipt, Result, SumUpClient};
use serde::Serialize;

/// Query parameters for retrieving receipts (per OpenAPI spec)
#[derive(Debug, Clone, Serialize)]
pub struct ReceiptRetrieveQuery {
    /// Merchant code (required)
    pub mid: String,
    /// Transaction event ID (optional, for refund receipts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_event_id: Option<i32>,
}

impl ReceiptRetrieveQuery {
    pub fn new(mid: impl Into<String>) -> Self {
        Self {
            mid: mid.into(),
            tx_event_id: None,
        }
    }

    pub fn tx_event_id(mut self, tx_event_id: i32) -> Self {
        self.tx_event_id = Some(tx_event_id);
        self
    }
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
