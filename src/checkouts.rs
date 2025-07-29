use crate::{SumUpClient, Result, CreateCheckoutRequest, Checkout, ProcessCheckoutRequest, DeletedCheckout, AvailablePaymentMethodsResponse, CheckoutListQuery};

impl SumUpClient {
    /// Lists created checkout resources according to the applied checkout_reference.
    ///
    /// # Arguments
    /// * `checkout_reference` - Unique ID of the payment checkout specified by the client application.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sumup_rs::SumUpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SumUpClient::new("your-api-key".to_string(), true)?;
    /// 
    /// // List all checkouts
    /// let checkouts = client.list_checkouts(None).await?;
    /// println!("Found {} checkouts", checkouts.len());
    /// 
    /// // List checkouts with specific reference
    /// let checkouts = client.list_checkouts(Some("order-123")).await?;
    /// println!("Found {} checkouts with reference 'order-123'", checkouts.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_checkouts(&self, checkout_reference: Option<&str>) -> Result<Vec<Checkout>> {
        let query = CheckoutListQuery {
            checkout_reference: checkout_reference.map(|s| s.to_string()),
            status: None,
            merchant_code: None,
            customer_id: None,
            limit: None,
            offset: None,
        };
        self.list_checkouts_with_query(&query).await
    }

    /// Lists created checkout resources with advanced query parameters.
    ///
    /// # Arguments
    /// * `query` - Query parameters for filtering and pagination
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sumup_rs::{SumUpClient, CheckoutListQuery};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SumUpClient::new("your-api-key".to_string(), true)?;
    /// 
    /// // Create a query to filter checkouts
    /// let query = CheckoutListQuery {
    ///     checkout_reference: Some("order-123".to_string()),
    ///     status: Some("PAID".to_string()),
    ///     merchant_code: Some("merchant123".to_string()),
    ///     customer_id: Some("customer456".to_string()),
    ///     limit: Some(10),
    ///     offset: Some(0),
    /// };
    /// 
    /// let checkouts = client.list_checkouts_with_query(&query).await?;
    /// println!("Found {} checkouts matching criteria", checkouts.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_checkouts_with_query(&self, query: &CheckoutListQuery) -> Result<Vec<Checkout>> {
        let mut url = self.build_url("/v0.1/checkouts")?;

        // Add query parameters
        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(ref checkout_ref) = query.checkout_reference {
                query_pairs.append_pair("checkout_reference", checkout_ref);
            }
            if let Some(ref status) = query.status {
                query_pairs.append_pair("status", status);
            }
            if let Some(ref merchant_code) = query.merchant_code {
                query_pairs.append_pair("merchant_code", merchant_code);
            }
            if let Some(ref customer_id) = query.customer_id {
                query_pairs.append_pair("customer_id", customer_id);
            }
            if let Some(limit) = query.limit {
                query_pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(offset) = query.offset {
                query_pairs.append_pair("offset", &offset.to_string());
            }
        }

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let checkouts = response.json::<Vec<Checkout>>().await?;
            Ok(checkouts)
        } else {
            self.handle_error(response).await
        }
    }

    /// Creates a new payment checkout resource.
    ///
    /// # Arguments
    /// * `body` - The request body containing the details for the new checkout.
    pub async fn create_checkout(&self, body: &CreateCheckoutRequest) -> Result<Checkout> {
        let url = self.build_url("/v0.1/checkouts")?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let checkout = response.json::<Checkout>().await?;
            Ok(checkout)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves an identified checkout resource.
    ///
    /// # Arguments
    /// * `checkout_id` - The unique ID of the checkout resource.
    pub async fn retrieve_checkout(&self, checkout_id: &str) -> Result<Checkout> {
        let url = self.build_url(&format!("/v0.1/checkouts/{}", checkout_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let checkout = response.json::<Checkout>().await?;
            Ok(checkout)
        } else {
            self.handle_error(response).await
        }
    }

    /// Processing a checkout will attempt to charge the provided payment instrument.
    ///
    /// # Arguments
    /// * `checkout_id` - The unique ID of the checkout resource to process.
    /// * `body` - The request body containing payment details.
    pub async fn process_checkout(
        &self,
        checkout_id: &str,
        body: &ProcessCheckoutRequest,
    ) -> Result<Checkout> {
        let url = self.build_url(&format!("/v0.1/checkouts/{}", checkout_id))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let checkout = response.json::<Checkout>().await?;
            Ok(checkout)
        } else {
            self.handle_error(response).await
        }
    }

    /// Deactivates an identified checkout resource.
    ///
    /// # Arguments
    /// * `checkout_id` - The unique ID of the checkout resource to deactivate.
    pub async fn deactivate_checkout(&self, checkout_id: &str) -> Result<DeletedCheckout> {
        let url = self.build_url(&format!("/v0.1/checkouts/{}", checkout_id))?;

        let response = self
            .http_client
            .delete(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let deleted_checkout = response.json::<DeletedCheckout>().await?;
            Ok(deleted_checkout)
        } else {
            self.handle_error(response).await
        }
    }

    /// Gets available payment methods for a merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The merchant's unique code.
    /// * `amount` - The transaction amount (optional).
    /// * `currency` - The transaction currency (optional).
    pub async fn get_available_payment_methods(
        &self,
        merchant_code: &str,
        amount: Option<f64>,
        currency: Option<&str>,
    ) -> Result<AvailablePaymentMethodsResponse> {
        let mut url = self.build_url(&format!("/v0.1/merchants/{}/payment-methods", merchant_code))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(amt) = amount {
                query_pairs.append_pair("amount", &amt.to_string());
            }
            if let Some(curr) = currency {
                query_pairs.append_pair("currency", curr);
            }
        }

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let methods = response.json::<AvailablePaymentMethodsResponse>().await?;
            Ok(methods)
        } else {
            self.handle_error(response).await
        }
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