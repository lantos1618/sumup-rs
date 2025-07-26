use crate::{SumUpClient, Result, Payout, PayoutListResponse};

impl SumUpClient {
    /// Lists all payouts for the authenticated merchant.
    pub async fn list_payouts(&self) -> Result<PayoutListResponse> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/me/payouts
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Lists payouts for a specific merchant.
    pub async fn list_merchant_payouts(&self, merchant_code: &str) -> Result<PayoutListResponse> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/merchants/{merchant_code}/payouts
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves an identified payout resource.
    pub async fn retrieve_payout(&self, payout_id: &str) -> Result<Payout> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameter: /v0.1/me/payouts/{payout_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves a payout for a specific merchant.
    pub async fn retrieve_merchant_payout(&self, merchant_code: &str, payout_id: &str) -> Result<Payout> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameters: /v0.1/merchants/{merchant_code}/payouts/{payout_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }
} 