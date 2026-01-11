use sumup_rs::{CreateCheckoutRequest, CreateCustomerRequest, PersonalDetails, SumUpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")?;
    let client = SumUpClient::new(api_key, true)?;

    // Get merchant info
    #[allow(deprecated)]
    let profile = client.get_merchant_profile().await?;
    println!("Merchant: {} ({})", profile.name, profile.merchant_code);

    // Create customer
    let customer = client
        .create_customer(&CreateCustomerRequest {
            customer_id: format!("cust_{}", chrono::Utc::now().timestamp()),
            personal_details: Some(PersonalDetails {
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
                email: Some("john@example.com".to_string()),
                ..Default::default()
            }),
        })
        .await?;
    println!("Customer: {}", customer.customer_id);

    // Create checkout
    let checkout = client
        .create_checkout(
            &CreateCheckoutRequest::new(
                format!("order_{}", chrono::Utc::now().timestamp()),
                9.99,
                &profile.currency,
                &profile.merchant_code,
            )
            .description("Test purchase")
            .customer_id(&customer.customer_id),
        )
        .await?;
    println!("Checkout: {} ({})", checkout.id, checkout.status);

    Ok(())
}
