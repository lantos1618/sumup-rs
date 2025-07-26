use crate::{SumUpClient, Result, CreateCheckoutRequest, Checkout, ProcessCheckoutRequest, DeletedCheckout, AvailablePaymentMethodsResponse};

impl SumUpClient {
    /// Lists created checkout resources according to the applied checkout_reference.
    pub async fn list_checkouts(&self, checkout_reference: Option<&str>) -> Result<Vec<Checkout>> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with query parameters
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Creates a new payment checkout resource.
    pub async fn create_checkout(&self, body: &CreateCheckoutRequest) -> Result<Checkout> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for POST /v0.1/checkouts
        // 2. Make POST request with Authorization header and JSON body
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves an identified checkout resource.
    pub async fn retrieve_checkout(&self, checkout_id: &str) -> Result<Checkout> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameter: /v0.1/checkouts/{id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }

    /// Processing a checkout will attempt to charge the provided payment instrument.
    pub async fn process_checkout(&self, checkout_id: &str, body: &ProcessCheckoutRequest) -> Result<Checkout> {
        // TODO: Implement the actual HTTP request logic for PUT /v0.1/checkouts/{id}
        unimplemented!();
    }

    /// Deactivates an identified checkout resource.
    pub async fn deactivate_checkout(&self, checkout_id: &str) -> Result<DeletedCheckout> {
        // TODO: Implement the actual HTTP request logic for DELETE /v0.1/checkouts/{id}
        unimplemented!();
    }

    /// Get payment methods available for the given merchant to use with a checkout.
    pub async fn get_available_payment_methods(
        &self,
        merchant_code: &str,
        amount: Option<f64>,
        currency: Option<&str>
    ) -> Result<AvailablePaymentMethodsResponse> {
        // TODO: Implement the actual HTTP request logic for GET /v0.1/merchants/{merchant_code}/payment-methods
        unimplemented!();
    }
} 