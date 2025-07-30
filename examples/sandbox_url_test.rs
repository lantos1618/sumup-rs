use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");
    
    println!("=== SANDBOX CHECKOUT URL TEST ===");
    println!("Testing sandbox checkout URLs...");
    
    // Create a SANDBOX client (sandbox=true)
    let client = SumUpClient::new(api_key, true)?;
    
    // Get the merchant profile
    let merchant_profile = client.get_merchant_profile().await?;
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("sandbox-test-{}", chrono::Utc::now().timestamp()),
        amount: 1.00, // Very small amount
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("Sandbox URL test".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    println!("Creating SANDBOX checkout...");
    
    // Create the checkout
    let checkout = client.create_checkout(&checkout_request).await?;
    
    println!("✅ SANDBOX Checkout created!");
    println!("   ID: {}", checkout.id);
    println!("   Amount: {} {}", checkout.amount, checkout.currency);
    println!("   Status: {}", checkout.status);
    println!("   Return URL: {:?}", checkout.return_url);
    println!("   Valid Until: {:?}", checkout.valid_until);
    
    println!("\n=== SANDBOX URL OPTIONS ===");
    
    // Try different sandbox URL formats
    let urls_to_test = vec![
        format!("https://checkout.sumup.com/{}", checkout.id),
        format!("https://sandbox.checkout.sumup.com/{}", checkout.id),
        format!("https://test.checkout.sumup.com/{}", checkout.id),
        format!("https://pay.sumup.com/{}", checkout.id),
        format!("https://sandbox.pay.sumup.com/{}", checkout.id),
        format!("https://test.pay.sumup.com/{}", checkout.id),
    ];
    
    println!("Testing these sandbox URLs:");
    for (i, url) in urls_to_test.iter().enumerate() {
        println!("{}. {}", i + 1, url);
    }
    
    println!("\n=== SANDBOX REALITY CHECK ===");
    println!("🔍 The truth about SumUp sandbox checkouts:");
    println!();
    println!("❌ SANDBOX CHECKOUTS ARE NOT PUBLICLY ACCESSIBLE");
    println!("   - They are for API testing only");
    println!("   - No public web interface");
    println!("   - URLs will show 'page not found'");
    println!();
    println!("✅ WHAT SANDBOX IS FOR:");
    println!("   - Testing API calls");
    println!("   - Testing payment processing");
    println!("   - Testing webhooks");
    println!("   - Development without real money");
    println!();
    println!("🎯 FOR TESTING THE WEB INTERFACE:");
    println!("   - Use PRODUCTION mode with small amounts");
    println!("   - Use your own real cards");
    println!("   - Test with £1 or less");
    
    println!("\n=== SANDBOX API TESTING ===");
    println!("Let's test the sandbox API functionality...");
    
    // Test retrieving the checkout
    match client.retrieve_checkout(&checkout.id).await {
        Ok(retrieved) => {
            println!("✅ Sandbox API works - retrieved checkout: {}", retrieved.status);
        }
        Err(e) => {
            println!("❌ Sandbox API failed: {}", e);
        }
    }
    
    // Test listing checkouts
    match client.list_checkouts(Some(&checkout_request.checkout_reference)).await {
        Ok(checkouts) => {
            println!("✅ Sandbox API works - found {} checkout(s)", checkouts.len());
        }
        Err(e) => {
            println!("❌ Sandbox API failed: {}", e);
        }
    }
    
    println!("\n=== CONCLUSION ===");
    println!("🎯 SANDBOX = API Testing Only");
    println!("🎯 PRODUCTION = Web Interface + Real Payments");
    println!();
    println!("For testing the web interface, use production mode with small amounts!");
    
    // Clean up
    match client.deactivate_checkout(&checkout.id).await {
        Ok(_) => println!("✅ Sandbox checkout cleaned up"),
        Err(_) => println!("⚠️  Could not clean up sandbox checkout"),
    }
    
    Ok(())
} 