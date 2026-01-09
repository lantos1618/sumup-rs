use crate::{
    AvailablePaymentMethodsResponse, Checkout, CheckoutListQuery, CreateCheckoutRequest,
    DeletedCheckout, ProcessCheckoutRequest, ProcessCheckoutResponse, Result, SumUpClient,
};

impl SumUpClient {
    /// Lists checkouts, optionally filtered by reference.
    pub async fn list_checkouts(&self, checkout_reference: Option<&str>) -> Result<Vec<Checkout>> {
        self.list_checkouts_with_query(&CheckoutListQuery {
            checkout_reference: checkout_reference.map(|s| s.to_string()),
            ..Default::default()
        }).await
    }

    /// Lists checkouts with query parameters.
    pub async fn list_checkouts_with_query(&self, query: &CheckoutListQuery) -> Result<Vec<Checkout>> {
        let mut url = self.build_url("/v0.1/checkouts")?;
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(ref v) = query.checkout_reference { pairs.append_pair("checkout_reference", v); }
            if let Some(ref v) = query.status { pairs.append_pair("status", &serde_json::to_string(v).unwrap().trim_matches('"').to_string()); }
            if let Some(ref v) = query.merchant_code { pairs.append_pair("merchant_code", v); }
            if let Some(ref v) = query.customer_id { pairs.append_pair("customer_id", v); }
            if let Some(v) = query.limit { pairs.append_pair("limit", &v.to_string()); }
            if let Some(v) = query.offset { pairs.append_pair("offset", &v.to_string()); }
        }
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Creates a new checkout.
    pub async fn create_checkout(&self, body: &CreateCheckoutRequest) -> Result<Checkout> {
        let url = self.build_url("/v0.1/checkouts")?;
        let response = self.http_client.post(url).bearer_auth(&self.api_key).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves a checkout by ID.
    pub async fn retrieve_checkout(&self, checkout_id: &str) -> Result<Checkout> {
        let url = self.build_url(&format!("/v0.1/checkouts/{}", checkout_id))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Processes a checkout (charges the payment instrument).
    /// Returns Success for immediate completion, or Accepted for 3DS redirect.
    pub async fn process_checkout(&self, checkout_id: &str, body: &ProcessCheckoutRequest) -> Result<ProcessCheckoutResponse> {
        let url = self.build_url(&format!("/v0.1/checkouts/{}", checkout_id))?;
        let response = self.http_client.put(url).bearer_auth(&self.api_key).json(body).send().await?;

        let status = response.status().as_u16();
        match status {
            200 | 202 => {
                let text = response.text().await.unwrap_or_default();
                if text.contains("next_step") {
                    Ok(ProcessCheckoutResponse::Accepted(serde_json::from_str(&text)?))
                } else {
                    Ok(ProcessCheckoutResponse::Success(serde_json::from_str(&text)?))
                }
            }
            _ => self.handle_error(response).await,
        }
    }

    /// Deactivates a checkout.
    pub async fn deactivate_checkout(&self, checkout_id: &str) -> Result<DeletedCheckout> {
        let url = self.build_url(&format!("/v0.1/checkouts/{}", checkout_id))?;
        let response = self.http_client.delete(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Gets available payment methods for a merchant.
    pub async fn get_available_payment_methods(&self, merchant_code: &str, amount: Option<f64>, currency: Option<&str>) -> Result<AvailablePaymentMethodsResponse> {
        let mut url = self.build_url(&format!("/v0.1/merchants/{}/payment-methods", merchant_code))?;
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(v) = amount { pairs.append_pair("amount", &v.to_string()); }
            if let Some(v) = currency { pairs.append_pair("currency", v); }
        }
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{CheckoutStatus, CreateCheckoutRequest, SumUpClient};
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_create_checkout_success() {
        let mock_server = MockServer::start().await;
        let request_body = CreateCheckoutRequest::new("test_ref_123", 10.50, "EUR", "M123")
            .description("A test checkout");

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

        Mock::given(method("POST"))
            .and(path("/v0.1/checkouts"))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
        let result = client.create_checkout(&request_body).await;

        assert!(result.is_ok());
        let checkout = result.unwrap();
        assert_eq!(checkout.id, "88fcf8de-304d-4820-8f1c-ec880290eb92");
        assert_eq!(checkout.status, CheckoutStatus::Pending);
    }

    #[tokio::test]
    async fn test_retrieve_checkout_success() {
        let mock_server = MockServer::start().await;
        let checkout_id = "88fcf8de-304d-4820-8f1c-ec880290eb92";

        let response_body = serde_json::json!({
            "id": checkout_id,
            "status": "PENDING",
            "checkout_reference": "test_ref_123",
            "amount": 10.50,
            "currency": "EUR",
            "merchant_code": "M123",
            "date": "2020-02-29T10:56:56+00:00",
            "transactions": []
        });

        Mock::given(method("GET"))
            .and(path(format!("/v0.1/checkouts/{}", checkout_id)))
            .and(header("Authorization", "Bearer test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
        let result = client.retrieve_checkout(checkout_id).await;

        assert!(result.is_ok());
        let checkout = result.unwrap();
        assert_eq!(checkout.id, checkout_id);
    }
}
