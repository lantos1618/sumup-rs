use crate::{SumUpClient, Result, CreateCheckoutRequest, Checkout, ProcessCheckoutRequest, DeletedCheckout, AvailablePaymentMethodsResponse, Error};

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
        // 1. Build URL for POST /v0.1/checkouts
        let url = self.build_url("/v0.1/checkouts")?;

        // 2. Make POST request
        let response = self.http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        // 3. Handle response
        if response.status().is_success() {
            let checkout = response.json::<Checkout>().await?;
            Ok(checkout)
        } else {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            Err(Error::ApiError { status, message })
        }
    }

    /// Retrieves an identified checkout resource.
    pub async fn retrieve_checkout(&self, checkout_id: &str) -> Result<Checkout> {
        // 1. Build URL with path parameter: /v0.1/checkouts/{id}
        let url = self.build_url(&format!("/v0.1/checkouts/{}", checkout_id))?;

        // 2. Make GET request
        let response = self.http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        // 3. Handle response
        if response.status().is_success() {
            let checkout = response.json::<Checkout>().await?;
            Ok(checkout)
        } else {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            Err(Error::ApiError { status, message })
        }
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

#[cfg(test)]
mod tests {
    use crate::{SumUpClient, CreateCheckoutRequest};
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, body_json, header};

    #[tokio::test]
    async fn test_create_checkout_success() {
        // 1. Arrange: Start a mock server
        let mock_server = MockServer::start().await;

        // The request body we expect our client to send
        let request_body = CreateCheckoutRequest {
            checkout_reference: "test_ref_123".to_string(),
            amount: 10.50,
            currency: "EUR".to_string(),
            merchant_code: "M123".to_string(),
            description: Some("A test checkout".to_string()),
            return_url: None,
            customer_id: None,
            purpose: None,
            redirect_url: None,
        };

        // The response body the mock server will return
        let response_body = serde_json::json!({
            "id": "88fcf8de-304d-4820-8f1c-ec880290eb92",
            "status": "PENDING",
            "checkout_reference": "test_ref_123",
            "amount": 10.50,
            "currency": "EUR",
            "merchant_code": "M123",
            "date": "2020-02-29T10:56:56+00:00",
            "description": "A test checkout",
            "transactions": []
        });

        // 2. Arrange: Set up the mock response
        Mock::given(method("POST"))
            .and(path("/v0.1/checkouts"))
            .and(header("Authorization", "Bearer test-api-key"))
            .and(body_json(&request_body))
            .respond_with(
                ResponseTemplate::new(201) // 201 Created
                    .set_body_json(&response_body)
            )
            .mount(&mock_server)
            .await;

        // 3. Act: Create a client pointing to the mock server and call the function
        let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
        let result = client.create_checkout(&request_body).await;

        // 4. Assert: Check if the result is what we expect
        assert!(result.is_ok());
        let checkout = result.unwrap();
        assert_eq!(checkout.id, "88fcf8de-304d-4820-8f1c-ec880290eb92");
        assert_eq!(checkout.status, "PENDING");
        assert_eq!(checkout.amount, 10.50);
    }

    #[tokio::test]
    async fn test_retrieve_checkout_success() {
        // 1. Arrange: Start a mock server
        let mock_server = MockServer::start().await;

        let checkout_id = "88fcf8de-304d-4820-8f1c-ec880290eb92";

        // The response body the mock server will return
        let response_body = serde_json::json!({
            "id": "88fcf8de-304d-4820-8f1c-ec880290eb92",
            "status": "PENDING",
            "checkout_reference": "test_ref_123",
            "amount": 10.50,
            "currency": "EUR",
            "merchant_code": "M123",
            "date": "2020-02-29T10:56:56+00:00",
            "description": "A test checkout",
            "transactions": []
        });

        // 2. Arrange: Set up the mock response
        Mock::given(method("GET"))
            .and(path(format!("/v0.1/checkouts/{}", checkout_id)))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(
                ResponseTemplate::new(200) // 200 OK
                    .set_body_json(&response_body)
            )
            .mount(&mock_server)
            .await;

        // 3. Act: Create a client pointing to the mock server and call the function
        let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
        let result = client.retrieve_checkout(checkout_id).await;

        // 4. Assert: Check if the result is what we expect
        assert!(result.is_ok());
        let checkout = result.unwrap();
        assert_eq!(checkout.id, checkout_id);
        assert_eq!(checkout.status, "PENDING");
        assert_eq!(checkout.amount, 10.50);
    }
} 