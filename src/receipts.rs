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
    /// 
    /// Note: The /v1.1/receipts list endpoint does not exist in the SumUp API.
    /// Only individual receipt retrieval is supported.
    #[deprecated(since = "0.2.0", note = "The /v1.1/receipts list endpoint does not exist. Use retrieve_receipt for individual receipts.")]
    pub async fn list_receipts(&self, _query: &ReceiptListQuery) -> Result<ReceiptListResponse> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v1.1/receipts list endpoint does not exist in the SumUp API. Only individual receipt retrieval is supported.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
    }

    /// Lists receipts for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `query` - Query parameters including required mid
    /// 
    /// Note: The /v1.1/merchants/{merchant_code}/receipts list endpoint does not exist in the SumUp API.
    /// Only individual receipt retrieval is supported.
    #[deprecated(since = "0.2.0", note = "The merchant receipts list endpoint does not exist. Use retrieve_merchant_receipt for individual receipts.")]
    pub async fn list_merchant_receipts(&self, _merchant_code: &str, _query: &ReceiptListQuery) -> Result<ReceiptListResponse> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v1.1/merchants/{merchant_code}/receipts list endpoint does not exist in the SumUp API. Only individual receipt retrieval is supported.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
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