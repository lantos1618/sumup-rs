//! Real API integration tests.
//!
//! Run with: `cargo test --features integration-tests -- --ignored`
//! Requires: SUMUP_API_SECRET_KEY environment variable

#![cfg(feature = "integration-tests")]

use sumup_rs::{Address, CreateCustomerRequest, PersonalDetails, SumUpClient};

fn get_client() -> Option<SumUpClient> {
    dotenv::from_filename(".env.local").ok();
    let api_key = std::env::var("SUMUP_API_SECRET_KEY").ok()?;
    SumUpClient::new(api_key, true).ok()
}

#[tokio::test]
#[ignore = "requires SUMUP_API_SECRET_KEY"]
async fn test_real_api_merchant_profile() {
    let client = get_client().expect("Need SUMUP_API_SECRET_KEY");

    #[allow(deprecated)]
    let profile = client.get_merchant_profile().await.expect("Failed to get profile");

    assert!(!profile.merchant_code.is_empty());
    assert!(!profile.name.is_empty());
    assert!(!profile.country.is_empty());
    assert!(!profile.currency.is_empty());
}

#[tokio::test]
#[ignore = "requires SUMUP_API_SECRET_KEY"]
async fn test_real_api_create_customer() {
    let client = get_client().expect("Need SUMUP_API_SECRET_KEY");

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

    let customer = client
        .create_customer(&customer_request)
        .await
        .expect("Failed to create customer");

    assert_eq!(customer.customer_id, customer_id);
}

#[tokio::test]
#[ignore = "requires SUMUP_API_SECRET_KEY"]
async fn test_real_api_list_memberships() {
    let client = get_client().expect("Need SUMUP_API_SECRET_KEY");

    let memberships = client.list_memberships().await.expect("Failed to list memberships");
    // Just verify it doesn't panic - might be empty for new accounts
    println!("Found {} memberships", memberships.len());
}
