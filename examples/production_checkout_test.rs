use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_KEY")
        .expect("SUMUP_API_KEY environment variable must be set");
    
    // Try production mode (sandbox=false)
    println!("Testing PRODUCTION checkout (sandbox=false)...");
    let client = SumUpClient::new(api_key.clone(), false)?;
    
    // Get the merchant profile
    let merchant_profile = client.get_merchant_profile().await?;
    println!("Using merchant code: {}", merchant_profile.merchant_code);
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("prod-test-{}", chrono::Utc::now().timestamp()),
        amount: 1.00, // Very small amount for testing
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("Production checkout test".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    println!("Creating production checkout...");
    
    // Create the checkout
    match client.create_checkout(&checkout_request).await {
        Ok(checkout) => {
            println!("✅ PRODUCTION Checkout created successfully!");
            println!("   ID: {}", checkout.id);
            println!("   Status: {}", checkout.status);
            println!("   Amount: {} {}", checkout.amount, checkout.currency);
            println!("   Reference: {:?}", checkout.checkout_reference);
            println!("   Return URL: {:?}", checkout.return_url);
            println!("   Valid Until: {:?}", checkout.valid_until);
            
            println!("\n=== PRODUCTION CHECKOUT URL ===");
            println!("Production URL:");
            println!("https://checkout.sumup.com/{}", checkout.id);
            
            println!("\n=== IMPORTANT ===");
            println!("⚠️  WARNING: This is a PRODUCTION checkout!");
            println!("   - Real money will be charged if used");
            println!("   - Use only for testing with real cards");
            println!("   - Consider deactivating after testing");
            
            // Deactivate immediately to prevent accidental use
            println!("\nDeactivating production checkout for safety...");
            match client.deactivate_checkout(&checkout.id).await {
                Ok(deleted) => {
                    println!("✅ Checkout deactivated: {}", deleted.status);
                }
                Err(e) => {
                    println!("❌ Failed to deactivate: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create production checkout: {}", e);
            println!("This might be because:");
            println!("- API key is sandbox-only");
            println!("- Account not activated for production");
            println!("- Missing production permissions");
        }
    }
    
    Ok(())
} 