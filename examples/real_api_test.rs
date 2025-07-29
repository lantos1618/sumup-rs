use sumup_rs::{SumUpClient, CreateCustomerRequest, PersonalDetails, Address};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env.local
    dotenv::from_filename(".env.local").ok();
    
    // Get API key from environment
    let api_key = std::env::var("SUMUP_API_KEY")
        .expect("SUMUP_API_KEY environment variable must be set");
    
    println!("Using API key: {}...", &api_key[..20]);
    
    // Create a client with the real API (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;
    
    println!("=== Testing SumUp API with Real Key ===\n");
    
    // Test 1: Get merchant profile
    println!("1. Testing merchant profile...");
    match client.get_merchant_profile().await {
        Ok(profile) => {
            println!("✅ Merchant profile retrieved successfully!");
            println!("   Merchant Code: {}", profile.merchant_code);
            println!("   Name: {}", profile.name);
            println!("   Email: {}", profile.email);
            println!("   Country: {}", profile.country);
            println!("   Currency: {}", profile.currency);
        }
        Err(e) => {
            println!("❌ Merchant profile failed: {}", e);
        }
    }
    
    // Test 2: Create a test customer
    println!("\n2. Testing create customer...");
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
            println!("✅ Customer created successfully!");
            println!("   Customer ID: {}", customer.customer_id);
            println!("   Email: {}", customer.personal_details.as_ref()
                .and_then(|pd| pd.email.as_ref())
                .unwrap_or(&"N/A".to_string()));
        }
        Err(e) => {
            println!("❌ Customer creation failed: {}", e);
        }
    }
    
    // Test 3: List merchants
    println!("\n3. Testing list merchants...");
    match client.list_merchants().await {
        Ok(merchants) => {
            println!("✅ List merchants successful!");
            println!("   Found {} merchants", merchants.len());
            for merchant in merchants {
                println!("   - {} ({})", merchant.name, merchant.merchant_code);
            }
        }
        Err(e) => {
            println!("❌ List merchants failed: {}", e);
        }
    }
    
    println!("\n=== Test completed ===");
    Ok(())
} 