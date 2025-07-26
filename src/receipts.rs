use crate::{SumUpClient, Result, Receipt, ReceiptListResponse};

impl SumUpClient {
    /// Lists all receipts for the authenticated merchant.
    pub async fn list_receipts(&self) -> Result<ReceiptListResponse> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/me/receipts
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Lists receipts for a specific merchant.
    pub async fn list_merchant_receipts(&self, merchant_code: &str) -> Result<ReceiptListResponse> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/merchants/{merchant_code}/receipts
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves an identified receipt resource.
    pub async fn retrieve_receipt(&self, receipt_id: &str) -> Result<Receipt> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameter: /v0.1/me/receipts/{receipt_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves a receipt for a specific merchant.
    pub async fn retrieve_merchant_receipt(&self, merchant_code: &str, receipt_id: &str) -> Result<Receipt> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameters: /v0.1/merchants/{merchant_code}/receipts/{receipt_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }
} 