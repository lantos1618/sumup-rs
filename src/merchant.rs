use crate::{SumUpClient, Result, Merchant, MerchantProfile};

impl SumUpClient {
    /// Retrieves the authenticated merchant's profile.
    pub async fn get_merchant_profile(&self) -> Result<MerchantProfile> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/me
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves a specific merchant's profile.
    pub async fn get_merchant(&self, merchant_code: &str) -> Result<Merchant> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameter: /v0.1/merchants/{merchant_code}
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Lists all merchants accessible to the authenticated user.
    pub async fn list_merchants(&self) -> Result<Vec<Merchant>> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/me/merchants
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }
} 