use crate::{SumUpClient, Result, Transaction, TransactionListResponse};

impl SumUpClient {
    /// Lists all transactions for the authenticated merchant.
    pub async fn list_transactions(&self) -> Result<TransactionListResponse> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/me/transactions
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Lists transactions for a specific merchant.
    pub async fn list_merchant_transactions(&self, merchant_code: &str) -> Result<TransactionListResponse> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/merchants/{merchant_code}/transactions
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves an identified transaction resource.
    pub async fn retrieve_transaction(&self, transaction_id: &str) -> Result<Transaction> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameter: /v0.1/me/transactions/{transaction_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves a transaction for a specific merchant.
    pub async fn retrieve_merchant_transaction(&self, merchant_code: &str, transaction_id: &str) -> Result<Transaction> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameters: /v0.1/merchants/{merchant_code}/transactions/{transaction_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }
} 