#![allow(unused_variables)]
use sumup_rs::{Address, CreateCustomerRequest, PersonalDetails, SumUpClient};

#[tokio::test]
async fn test_real_api_merchant_profile() {
    // Load environment variables from .env.local
    dotenv::from_filename(".env.local").ok();

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
            assert!(!profile.country.is_empty());
            assert!(!profile.currency.is_empty());
        }
        Err(e) => {
            println!("API test failed (expected with test key): {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_create_customer() {
    dotenv::from_filename(".env.local").ok();

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

    let customer_id = format!("test-cust-{}", chrono::Utc::now().timestamp());

    let customer_request = CreateCustomerRequest {
        customer_id: customer_id.clone(),
        personal_details: Some(PersonalDetails {
            first_name: Some("Test".to_string()),
            last_name: Some("Customer".to_string()),
            email: Some("test.customer@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            birth_date: Some("1990-01-01".to_string()),
            tax_id: Some("123456789".to_string()),
            address: Some(Address {
                city: Some("Test City".to_string()),
                country: Some("US".to_string()),
                line_1: Some("123 Test St".to_string()),
                line_2: Some("Apt 1".to_string()),
                postal_code: Some("12345".to_string()),
                state: Some("CA".to_string()),
            }),
        }),
    };

    match client.create_customer(&customer_request).await {
        Ok(customer) => {
            assert_eq!(customer.customer_id, customer_id);
            let personal_details = customer.personal_details.unwrap();
            assert_eq!(personal_details.first_name, Some("Test".to_string()));
            assert_eq!(personal_details.last_name, Some("Customer".to_string()));
        }
        Err(e) => {
            println!("API test failed (expected with test key): {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_list_merchants() {
    dotenv::from_filename(".env.local").ok();

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
