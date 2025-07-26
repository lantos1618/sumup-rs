use crate::{SumUpClient, Result, Receipt, ReceiptListResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ReceiptListQuery {
    pub mid: String, // Required: Merchant ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReceiptRetrieveQuery {
    pub mid: String, // Required: Merchant ID
}

impl SumUpClient {
    /// Lists all receipts for the authenticated merchant.
    ///
    /// # Arguments
    /// * `query` - Query parameters including required mid
    pub async fn list_receipts(&self, query: &ReceiptListQuery) -> Result<ReceiptListResponse> {
        let url = self.build_url("/v1.1/receipts")?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .query(query)
            .send()
            .await?;

        if response.status().is_success() {
            let receipts = response.json::<ReceiptListResponse>().await?;
            Ok(receipts)
        } else {
            self.handle_error(response).await
        }
    }

    /// Lists receipts for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `query` - Query parameters including required mid
    pub async fn list_merchant_receipts(&self, merchant_code: &str, query: &ReceiptListQuery) -> Result<ReceiptListResponse> {
        let url = self.build_url(&format!("/v1.1/merchants/{}/receipts", merchant_code))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .query(query)
            .send()
            .await?;

        if response.status().is_success() {
            let receipts = response.json::<ReceiptListResponse>().await?;
            Ok(receipts)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves an identified receipt resource.
    ///
    /// # Arguments
    /// * `receipt_id` - The unique receipt identifier
    /// * `query` - Query parameters including required mid
    pub async fn retrieve_receipt(&self, receipt_id: &str, query: &ReceiptRetrieveQuery) -> Result<Receipt> {
        let url = self.build_url(&format!("/v1.1/receipts/{}", receipt_id))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .query(query)
            .send()
            .await?;

        if response.status().is_success() {
            let receipt = response.json::<Receipt>().await?;
            Ok(receipt)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves a receipt for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `receipt_id` - The unique receipt identifier
    /// * `query` - Query parameters including required mid
    pub async fn retrieve_merchant_receipt(&self, merchant_code: &str, receipt_id: &str, query: &ReceiptRetrieveQuery) -> Result<Receipt> {
        let url = self.build_url(&format!("/v1.1/merchants/{}/receipts/{}", merchant_code, receipt_id))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .query(query)
            .send()
            .await?;

        if response.status().is_success() {
            let receipt = response.json::<Receipt>().await?;
            Ok(receipt)
        } else {
            self.handle_error(response).await
        }
    }
} 