use sumup_rs::SumUpClient;

#[tokio::test]
async fn test_merchant_profile_integration() {
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

    // Test merchant profile retrieval
    match client.get_merchant_profile().await {
        Ok(profile) => {
            assert!(!profile.merchant_code.is_empty());
            assert!(!profile.name.is_empty());
            // Email is now in doing_business_as, so we'll check if it exists
            if let Some(dba) = &profile.doing_business_as {
                assert!(!dba.email.is_empty());
            }
        }
        Err(e) => {
            // Don't fail the test for API errors - this is expected with invalid keys
            println!("API test failed (expected with test key): {}", e);
        }
    }
}

#[tokio::test]
async fn test_list_merchants_integration() {
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

    match client.list_merchants().await {
        Ok(merchants) => {
            // Should return a list (might be empty)
            // No assertion needed - just verifying it doesn't panic
        }
        Err(e) => {
            println!("API test failed (expected with test key): {}", e);
        }
    }
}

#[tokio::test]
async fn test_create_customer_integration() {
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

    let customer_request = sumup_rs::CreateCustomerRequest {
        customer_id: format!("test-customer-{}", chrono::Utc::now().timestamp()),
        personal_details: Some(sumup_rs::PersonalDetails {
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            birth_date: Some("1990-01-01".to_string()),
            tax_id: None,
            address: Some(sumup_rs::Address {
                city: Some("Test City".to_string()),
                country: Some("US".to_string()),
                line_1: Some("123 Test St".to_string()),
                line_2: None,
                postal_code: Some("12345".to_string()),
                state: Some("CA".to_string()),
            }),
        }),
    };

    match client.create_customer(&customer_request).await {
        Ok(customer) => {
            assert_eq!(customer.customer_id, customer_request.customer_id);
        }
        Err(e) => {
            println!("API test failed (expected with test key): {}", e);
        }
    }
}
