use sumup_rs::{SumUpClient, CreateCheckoutRequest, ProcessCheckoutRequest, CardDetails};

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
        checkout_reference: format!("real-payment-{}", chrono::Utc::now().timestamp()),
        amount: 10.00, // Small amount for testing
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("Real payment test with 3DS".to_string()),
        return_url: Some("https://your-app.com/payment-return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    println!("Creating checkout for real payment...");
    
    // Create the checkout
    match client.create_checkout(&checkout_request).await {
        Ok(checkout) => {
            println!("✅ Checkout created successfully!");
            println!("   ID: {}", checkout.id);
            println!("   Status: {}", checkout.status);
            println!("   Amount: {} {}", checkout.amount, checkout.currency);
            println!("   Reference: {:?}", checkout.checkout_reference);
            
            println!("\n=== REAL PAYMENT FLOW ===");
            println!("To complete a real payment, you would:");
            println!("1. Present this checkout to your customer");
            println!("2. Customer enters their real card details");
            println!("3. If 3DS is required, they'll be redirected to their bank");
            println!("4. After authentication, they return to your return_url");
            
            println!("\n=== CHECKOUT URL ===");
            println!("Your customer would use this checkout URL:");
            println!("https://checkout.sumup.com/{}", checkout.id);
            
            println!("\n=== RETURN URL ===");
            println!("After payment, customer returns to:");
            println!("{}", checkout_request.return_url.as_ref().unwrap());
            
            println!("\n=== WEBHOOK NOTIFICATION ===");
            println!("SumUp will also send webhook notifications to your server");
            println!("when payment status changes (pending, successful, failed)");
            
            // Show how to retrieve the checkout to check status
            println!("\n=== CHECKING PAYMENT STATUS ===");
            println!("You can check payment status by calling:");
            println!("client.retrieve_checkout(\"{}\")", checkout.id);
            
            // Don't deactivate this checkout - let it expire naturally
            // or wait for customer to complete payment
            println!("\n=== IMPORTANT ===");
            println!("This checkout will remain active for customer payment");
            println!("Don't deactivate it unless you want to cancel the payment");
            
            // Example of what the response would look like after 3DS
            println!("\n=== EXPECTED 3DS FLOW ===");
            println!("1. Customer enters card details");
            println!("2. If 3DS required: Redirect to bank's 3DS page");
            println!("3. Customer authenticates with bank");
            println!("4. Bank redirects back to your return_url");
            println!("5. SumUp sends webhook with final status");
            println!("6. You can query checkout status to confirm");
        }
        Err(e) => {
            println!("❌ Failed to create checkout: {}", e);
        }
    }
    
    Ok(())
} 