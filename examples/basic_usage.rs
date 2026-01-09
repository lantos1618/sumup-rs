use sumup_rs::{
    Address, CreateCheckoutRequest, CreateCustomerRequest, PersonalDetails, SumUpClient,
    TransactionHistoryQuery,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("Please set SUMUP_API_SECRET_KEY environment variable");

    let client = SumUpClient::new(api_key, true)?;

    println!("SumUp API Client Example");
    println!("========================\n");

    // 1. Get merchant profile first (needed for other operations)
    println!("1. Getting merchant profile...");
    let profile = client.get_merchant_profile().await?;
    println!("   Merchant: {} ({})", profile.name, profile.merchant_code);

    // 2. Create a customer
    println!("\n2. Creating a customer...");
    let customer_request = CreateCustomerRequest {
        customer_id: format!("cust_{}", chrono::Utc::now().timestamp()),
        personal_details: Some(PersonalDetails {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some("john.doe@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            birth_date: None,
            tax_id: None,
            address: Some(Address {
                city: Some("Berlin".to_string()),
                country: Some("DE".to_string()),
                line_1: Some("123 Main St".to_string()),
                line_2: None,
                postal_code: Some("10115".to_string()),
                state: None,
            }),
        }),
    };
    let customer = client.create_customer(&customer_request).await?;
    println!("   Created customer: {}", customer.customer_id);

    // 3. Create a checkout
    println!("\n3. Creating a checkout...");
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("order_{}", chrono::Utc::now().timestamp()),
        amount: 9.99,
        currency: profile.currency.clone(),
        merchant_code: profile.merchant_code.clone(),
        description: Some("Test purchase".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: Some(customer.customer_id.clone()),
        purpose: None,
        redirect_url: None,
    };
    let checkout = client.create_checkout(&checkout_request).await?;
    println!("   Created checkout: {} ({})", checkout.id, checkout.status);

    // 4. List recent transactions
    println!("\n4. Listing transactions...");
    let query = TransactionHistoryQuery {
        limit: Some(5),
        order: Some("desc"),
        newest_time: None,
        oldest_time: None,
    };
    let transactions = client
        .list_transactions_history(&profile.merchant_code, &query)
        .await?;
    println!("   Found {} transactions", transactions.items.len());

    println!("\nDone!");
    Ok(())
}
