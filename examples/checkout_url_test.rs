use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_KEY")
        .expect("SUMUP_API_KEY environment variable must be set");
    
    // Create a client (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;
    
    // Get the merchant profile to use the correct merchant code
    let merchant_profile = client.get_merchant_profile().await?;
    println!("Using merchant code: {}", merchant_profile.merchant_code);
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("url-test-{}", chrono::Utc::now().timestamp()),
        amount: 5.00, // Small amount for testing
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("Testing checkout URL format".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    println!("Creating fresh checkout...");
    
    // Create the checkout
    let checkout = client.create_checkout(&checkout_request).await?;
    
    println!("✅ Checkout created successfully!");
    println!("   ID: {}", checkout.id);
    println!("   Status: {}", checkout.status);
    println!("   Amount: {} {}", checkout.amount, checkout.currency);
    println!("   Reference: {:?}", checkout.checkout_reference);
    println!("   Return URL: {:?}", checkout.return_url);
    println!("   Valid Until: {:?}", checkout.valid_until);
    
    println!("\n=== CHECKOUT URL OPTIONS ===");
    
    // Option 1: Standard SumUp checkout URL
    println!("1. Standard SumUp Checkout URL:");
    println!("   https://checkout.sumup.com/{}", checkout.id);
    
    // Option 2: Alternative format (if exists)
    println!("2. Alternative format:");
    println!("   https://pay.sumup.com/{}", checkout.id);
    
    // Option 3: Check if there's a redirect_url in the response
    if let Some(redirect_url) = &checkout.redirect_url {
        println!("3. Redirect URL from API response:");
        println!("   {}", redirect_url);
    }
    
    // Option 4: Sandbox-specific URL
    println!("4. Sandbox URL (if different):");
    println!("   https://sandbox.checkout.sumup.com/{}", checkout.id);
    
    println!("\n=== TESTING URLS ===");
    println!("Try these URLs in your browser:");
    println!("- https://checkout.sumup.com/{}", checkout.id);
    println!("- https://pay.sumup.com/{}", checkout.id);
    println!("- https://sandbox.checkout.sumup.com/{}", checkout.id);
    
    println!("\n=== IMPORTANT NOTES ===");
    println!("- This is a SANDBOX checkout (test environment)");
    println!("- Sandbox URLs might not be publicly accessible");
    println!("- The checkout will expire after a short time");
    println!("- For production, use sandbox=false in client creation");
    
    // Check if we're in sandbox mode
    println!("\n=== SANDBOX STATUS ===");
    println!("Current mode: SANDBOX (test environment)");
    println!("For production URLs, change client creation to:");
    println!("SumUpClient::new(api_key, false)");
    
    Ok(())
} 