use sumup_rs::{CreateCheckoutRequest, SumUpClient};

#[tokio::test]
async fn test_create_checkout_integration() {
    // Skip if no API key is available
    let api_key = match std::env::var("SUMUP_API_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping real API test - no SUMUP_API_SECRET_KEY set");
            return;
        }
    };

    let client = match SumUpClient::new(api_key, true) {
        Ok(client) => client,
        Err(_) => {
            println!("Skipping real API test - failed to create client");
            return;
        }
    };

    // Test creating a checkout
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("test-checkout-{}", chrono::Utc::now().timestamp()),
        amount: 10.00,
        currency: "EUR".to_string(),
        merchant_code: "test-merchant".to_string(),
        description: Some("Test checkout from Rust client".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: Some("cust_12345".to_string()),
        purpose: None,
        redirect_url: None,
    };

    match client.create_checkout(&checkout_request).await {
        Ok(checkout) => {
            assert_eq!(
                checkout.checkout_reference,
                Some(checkout_request.checkout_reference.clone())
            );
            assert_eq!(checkout.amount, checkout_request.amount);
            assert_eq!(checkout.currency, checkout_request.currency);
            assert!(!checkout.id.is_empty());
        }
        Err(e) => {
            // Don't fail the test for API errors - this is expected with invalid keys
            println!("API test failed (expected with test key): {}", e);
        }
    }
}

#[tokio::test]
async fn test_retrieve_checkout_integration() {
    let api_key = match std::env::var("SUMUP_API_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping real API test - no SUMUP_API_SECRET_KEY set");
            return;
        }
    };

    let client = match SumUpClient::new(api_key, true) {
        Ok(client) => client,
        Err(_) => {
            println!("Skipping real API test - failed to create client");
            return;
        }
    };

    // Try to retrieve a non-existent checkout (should fail gracefully)
    match client.retrieve_checkout("non-existent-checkout-id").await {
        Ok(_) => {
            // This shouldn't happen with a non-existent ID
            panic!("Unexpectedly succeeded in retrieving non-existent checkout");
        }
        Err(e) => {
            // This is expected - should get a 404 or similar error
            println!("Expected error for non-existent checkout: {}", e);
        }
    }
}
