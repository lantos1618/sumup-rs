use crate::{SumUpClient, Result, Customer, CreateCustomerRequest, UpdateCustomerRequest, PaymentInstrument};

impl SumUpClient {
    /// Lists all customers for the authenticated merchant.
    pub async fn list_customers(&self) -> Result<Vec<Customer>> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for GET /v0.1/customers
        // 2. Make GET request with Authorization header
        // 3. Deserialize response
        unimplemented!();
    }

    /// Creates a new customer resource.
    pub async fn create_customer(&self, body: &CreateCustomerRequest) -> Result<Customer> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL for POST /v0.1/customers
        // 2. Make POST request with Authorization header and JSON body
        // 3. Deserialize response
        unimplemented!();
    }

    /// Retrieves an identified customer resource.
    pub async fn retrieve_customer(&self, customer_id: &str) -> Result<Customer> {
        // TODO: Implement the actual HTTP request logic
        // 1. Build URL with path parameter: /v0.1/customers/{customer_id}
        // 2. Make GET request
        // 3. Deserialize response
        unimplemented!();
    }

    /// Updates an identified customer resource.
    pub async fn update_customer(&self, customer_id: &str, body: &UpdateCustomerRequest) -> Result<Customer> {
        // TODO: Implement the actual HTTP request logic for PUT /v0.1/customers/{customer_id}
        unimplemented!();
    }

    /// Deletes an identified customer resource.
    pub async fn delete_customer(&self, customer_id: &str) -> Result<()> {
        // TODO: Implement the actual HTTP request logic for DELETE /v0.1/customers/{customer_id}
        unimplemented!();
    }

    /// Lists payment instruments for a customer.
    pub async fn list_customer_payment_instruments(&self, customer_id: &str) -> Result<Vec<PaymentInstrument>> {
        // TODO: Implement the actual HTTP request logic for GET /v0.1/customers/{customer_id}/payment-instruments
        unimplemented!();
    }

    /// Creates a payment instrument for a customer.
    pub async fn create_customer_payment_instrument(&self, customer_id: &str, body: &serde_json::Value) -> Result<PaymentInstrument> {
        // TODO: Implement the actual HTTP request logic for POST /v0.1/customers/{customer_id}/payment-instruments
        unimplemented!();
    }

    /// Retrieves a payment instrument for a customer.
    pub async fn retrieve_customer_payment_instrument(&self, customer_id: &str, token: &str) -> Result<PaymentInstrument> {
        // TODO: Implement the actual HTTP request logic for GET /v0.1/customers/{customer_id}/payment-instruments/{token}
        unimplemented!();
    }

    /// Deletes a payment instrument for a customer.
    pub async fn delete_customer_payment_instrument(&self, customer_id: &str, token: &str) -> Result<()> {
        // TODO: Implement the actual HTTP request logic for DELETE /v0.1/customers/{customer_id}/payment-instruments/{token}
        unimplemented!();
    }
} 