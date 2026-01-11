//! Integration tests that require a real SumUp API key.
//!
//! Run with: `cargo test --features integration-tests -- --ignored`
//! Requires: SUMUP_API_SECRET_KEY environment variable

#![cfg(feature = "integration-tests")]

use sumup_rs::SumUpClient;

fn get_client() -> Option<SumUpClient> {
    let api_key = std::env::var("SUMUP_API_SECRET_KEY").ok()?;
    SumUpClient::new(api_key, true).ok()
}

#[tokio::test]
#[ignore = "requires SUMUP_API_SECRET_KEY"]
async fn test_merchant_profile_integration() {
    let client = get_client().expect("Need SUMUP_API_SECRET_KEY");

    #[allow(deprecated)]
    let profile = client.get_merchant_profile().await.expect("Failed to get profile");

    assert!(!profile.merchant_code.is_empty());
    assert!(!profile.name.is_empty());
}

#[tokio::test]
#[ignore = "requires SUMUP_API_SECRET_KEY"]
async fn test_list_memberships_integration() {
    let client = get_client().expect("Need SUMUP_API_SECRET_KEY");

    let memberships = client.list_memberships().await.expect("Failed to list memberships");
    // Should return a list (might be empty for new accounts)
    assert!(memberships.len() >= 0);
}

#[tokio::test]
#[ignore = "requires SUMUP_API_SECRET_KEY"]
async fn test_create_customer_integration() {
    let client = get_client().expect("Need SUMUP_API_SECRET_KEY");

    let customer_request = sumup_rs::CreateCustomerRequest {
        customer_id: format!("test-customer-{}", chrono::Utc::now().timestamp()),
        personal_details: Some(sumup_rs::PersonalDetails {
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            ..Default::default()
        }),
    };

    let customer = client
        .create_customer(&customer_request)
        .await
        .expect("Failed to create customer");

    assert_eq!(customer.customer_id, customer_request.customer_id);
}
