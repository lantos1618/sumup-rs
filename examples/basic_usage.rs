use sumup_rs::{SumUpClient, CreateCheckoutRequest, PersonalDetails, Address, CreateCustomerRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client (use sandbox for testing)
    let client = SumUpClient::new("your-api-key".to_string(), true)?;
    
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
        personal_details: Some(personal_details) 
    };
    // Note: This will panic with unimplemented!() until the HTTP logic is implemented
    // let customer = client.create_customer(&customer_request).await?;
    // println!("Created customer: {:?}", customer);
    println!("Customer creation would be implemented here");
    
    // Example 2: Create a checkout
    println!("\n2. Creating a checkout...");
    let checkout_request = CreateCheckoutRequest {
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
    // Note: This will panic with unimplemented!() until the HTTP logic is implemented
    // let transactions = client.list_transactions().await?;
    // println!("Found {} transactions", transactions.transactions.len());
    println!("Transaction listing would be implemented here");
    
    // Example 4: Get merchant profile
    println!("\n4. Getting merchant profile...");
    // Note: This will panic with unimplemented!() until the HTTP logic is implemented
    // let profile = client.get_merchant_profile().await?;
    // println!("Merchant profile: {:?}", profile);
    println!("Merchant profile retrieval would be implemented here");
    
    println!("\nExample completed successfully!");
    println!("Note: All API calls are currently unimplemented and would panic.");
    println!("To see actual functionality, implement the HTTP logic in each module.");
    
    Ok(())
} 