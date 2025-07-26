use crate::{SumUpClient, Result, Reader, ReaderListResponse};

impl SumUpClient {
    /// Lists all readers for the authenticated merchant.
    pub async fn list_readers(&self) -> Result<ReaderListResponse> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/me/readers
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Lists readers for a specific merchant.
    pub async fn list_merchant_readers(&self, merchant_code: &str) -> Result<ReaderListResponse> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/merchants/{merchant_code}/readers
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves an identified reader resource.
    pub async fn retrieve_reader(&self, reader_id: &str) -> Result<Reader> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameter: /v0.1/me/readers/{reader_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves a reader for a specific merchant.
    pub async fn retrieve_merchant_reader(&self, merchant_code: &str, reader_id: &str) -> Result<Reader> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameters: /v0.1/merchants/{merchant_code}/readers/{reader_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }
} 