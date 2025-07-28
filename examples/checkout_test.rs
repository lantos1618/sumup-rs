use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("SUMUP_API_KEY")
        .expect("Please set SUMUP_API_KEY environment variable");
    
    // Create a client (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;
    
    println!("=== SumUp Checkout Test ===\n");
    
    // Test creating a checkout
    println!("Creating a test checkout...");
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("test-checkout-{}", chrono::Utc::now().timestamp()),
        amount: 10.00,
        currency: "EUR".to_string(),
        merchant_code: "test-merchant".to_string(),
        description: Some("Test checkout from Rust client".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: Some("cust_12345".to_string()),
        purpose: None,
        redirect_url: None,
    };
    
    match client.create_checkout(&checkout_request).await {
        Ok(checkout) => {
            println!("✅ Checkout created successfully!");
            println!("   Checkout ID: {}", checkout.id);
            println!("   Status: {}", checkout.status);
            println!("   Amount: {} {}", checkout.amount, checkout.currency);
            println!("   Reference: {}", checkout.checkout_reference.as_deref().unwrap_or("N/A"));
            if let Some(redirect_url) = &checkout.redirect_url {
                println!("   Redirect URL: {}", redirect_url);
            }
        }
        Err(e) => {
            println!("❌ Failed to create checkout: {}", e);
            // Print more details about the error
            if let sumup_rs::Error::ApiError { status, body } = &e {
                println!("   Status: {}", status);
                if let Some(detail) = &body.detail {
                    println!("   Detail: {}", detail);
                }
                if let Some(error_code) = &body.error_code {
                    println!("   Error Code: {}", error_code);
                }
                if let Some(param) = &body.param {
                    println!("   Parameter: {}", param);
                }
                println!("   Full error body: {:?}", body);
            }
        }
    }
    
    println!("\n=== Checkout Test Complete ===");
    Ok(())
} 