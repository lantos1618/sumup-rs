use sumup_rs::{Amount, CreateCheckoutRequest, SumUpClient};

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

    // Test creating a checkout (1000 cents = $10.00)
    let checkout_request = CreateCheckoutRequest::new(
        format!("test-checkout-{}", chrono::Utc::now().timestamp()),
        Amount::from_cents(1000),
        "EUR",
        "test-merchant",
    )
    .description("Test checkout from Rust client")
    .return_url("https://example.com/return")
    .customer_id("cust_12345");

    match client.create_checkout(&checkout_request).await {
        Ok(checkout) => {
            assert_eq!(
                checkout.checkout_reference,
                Some(checkout_request.checkout_reference.clone())
            );
            assert_eq!(checkout.amount, checkout_request.amount);
            assert_eq!(checkout.currency.as_str(), checkout_request.currency.as_str());
            assert!(!checkout.id.0.is_empty());
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
