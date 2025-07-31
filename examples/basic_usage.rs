use sumup_rs::{
    Address, CreateCheckoutRequest, CreateCustomerRequest, PersonalDetails, SumUpClient,
    TransactionHistoryQuery,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("Please set SUMUP_API_SECRET_KEY environment variable");

    // Create a client (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;

    println!("SumUp API Client Example");
    println!("========================");

    // Example 1: Create a customer
    println!("\n1. Creating a customer...");
    let personal_details = PersonalDetails {
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("john.doe@example.com".to_string()),
        phone: Some("+1234567890".to_string()),
        birth_date: Some("1990-01-01".to_string()),
        tax_id: Some("123456789".to_string()),
        address: Some(Address {
            city: Some("New York".to_string()),
            country: Some("US".to_string()),
            line_1: Some("123 Main St".to_string()),
            line_2: Some("Apt 4B".to_string()),
            postal_code: Some("10001".to_string()),
            state: Some("NY".to_string()),
        }),
    };

    let customer_request = CreateCustomerRequest {
        customer_id: "cust_12345".to_string(),
        personal_details: Some(personal_details),
    };
    let customer = client.create_customer(&customer_request).await?;
    println!("Created customer: {:?}", customer);

    // Example 2: Create a checkout
    println!("\n2. Creating a checkout...");
    let _checkout_request = CreateCheckoutRequest {
        checkout_reference: "order-123".to_string(),
        amount: 29.99,
        currency: "EUR".to_string(),
        merchant_code: "your-merchant-code".to_string(),
        description: Some("Coffee and pastry".to_string()),
        return_url: Some("https://your-site.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };

    // Note: This will panic with unimplemented!() until the HTTP logic is implemented
    // let checkout = client.create_checkout(&checkout_request).await?;
    // println!("Created checkout: {:?}", checkout);
    println!("Checkout creation would be implemented here");

    // Example 3: List transactions
    println!("\n3. Listing transactions...");
    let query = TransactionHistoryQuery {
        limit: Some(10),
        order: Some("desc"),
        newest_time: None,
        oldest_time: None,
    };
    let transactions = client
        .list_transactions_history("your-merchant-code", &query)
        .await?;
    println!("Found {} transactions", transactions.items.len());

    // Example 4: Get merchant profile
    println!("\n4. Getting merchant profile...");
    let profile = client.get_merchant_profile().await?;
    println!("Merchant profile: {:?}", profile);

    println!("\nExample completed successfully!");
    println!("Note: Customer, Transaction, and Merchant APIs are now fully implemented.");
    println!("Checkout API is still in progress.");

    Ok(())
}
