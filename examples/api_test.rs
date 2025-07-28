use sumup_rs::SumUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("SUMUP_API_KEY")
        .expect("Please set SUMUP_API_KEY environment variable");
    
    let client = SumUpClient::new(api_key, true)?;
    
    println!("=== SumUp API Test ===\n");
    
    // Test 1: Get merchant profile
    println!("1. Testing merchant profile...");
    match client.get_merchant_profile().await {
        Ok(profile) => {
            println!("✅ Merchant profile retrieved successfully!");
            println!("   Merchant Code: {}", profile.merchant_code);
            println!("   Name: {}", profile.name);
            println!("   Email: {}", profile.email);
        }
        Err(e) => {
            println!("❌ Merchant profile failed: {}", e);
        }
    }
    
    // Test 2: List merchants
    println!("\n2. Testing list merchants...");
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
    
    // Test 3: Create a test customer
    println!("\n3. Testing create customer...");
    let customer_request = sumup_rs::CreateCustomerRequest {
        customer_id: "test-customer-123".to_string(),
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
            println!("✅ Create customer successful!");
            println!("   Customer ID: {}", customer.customer_id);
            println!("   Email: {}", customer.personal_details.as_ref()
                .and_then(|pd| pd.email.as_ref())
                .unwrap_or(&"N/A".to_string()));
        }
        Err(e) => {
            println!("❌ Create customer failed: {}", e);
        }
    }
    
    // Test 4: List payouts (with date range)
    println!("\n4. Testing list payouts...");
    let payout_query = sumup_rs::payouts::PayoutListQuery {
        start_date: "2023-01-01".to_string(),
        end_date: "2024-12-31".to_string(),
        limit: Some(5),
        offset: Some(0),
    };
    
    match client.list_payouts(&payout_query).await {
        Ok(payouts) => {
            println!("✅ List payouts successful!");
            println!("   Found {} payouts", payouts.payouts.len());
            for payout in &payouts.payouts {
                println!("   - {}: {} {} ({})", 
                    payout.id, payout.amount, payout.currency, payout.status);
            }
        }
        Err(e) => {
            println!("❌ List payouts failed: {}", e);
        }
    }
    
    // Test 5: List readers
    println!("\n5. Testing list readers...");
    match client.list_readers().await {
        Ok(readers) => {
            println!("✅ List readers successful!");
            println!("   Found {} readers", readers.readers.len());
            for reader in &readers.readers {
                println!("   - {} ({}) - Status: {}", 
                    reader.serial_number, reader.id, reader.status);
            }
        }
        Err(e) => {
            println!("❌ List readers failed: {}", e);
        }
    }
    
    println!("\n=== API Test Complete ===");
    Ok(())
} 