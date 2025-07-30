use sumup_rs::{SumUpClient, CreateCheckoutRequest, ProcessCheckoutRequest, CardDetails};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");
    
    // Create a client (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;
    
    // Get the merchant profile to use the correct merchant code
    let merchant_profile = client.get_merchant_profile().await?;
    println!("Using merchant code: {}", merchant_profile.merchant_code);
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("3ds-test-{}", chrono::Utc::now().timestamp()),
        amount: 15.00,
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("3DS Payment Test".to_string()),
        return_url: Some("https://your-app.com/payment-success".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    println!("Creating checkout for 3DS payment test...");
    
    // Create the checkout
    let checkout = client.create_checkout(&checkout_request).await?;
    println!("✅ Checkout created: {}", checkout.id);
    
    // Now simulate processing with a card that might trigger 3DS
    // In real usage, this would be the customer's actual card details
    let process_request = ProcessCheckoutRequest {
        payment_type: "card".to_string(),
        installments: None,
        card: Some(CardDetails {
            // This is a test card that might trigger 3DS in some cases
            // In production, this would be the customer's real card
            number: "4000000000003220".to_string(), // Test card that may require 3DS
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvc: "123".to_string(),
            name: Some("Test Customer".to_string()),
        }),
        token: None,
        customer_id: None,
        personal_details: None,
    };
    
    println!("\n=== PROCESSING PAYMENT ===");
    println!("Attempting to process payment...");
    
    match client.process_checkout(&checkout.id, &process_request).await {
        Ok(processed_checkout) => {
            println!("✅ Payment processing response received!");
            println!("   Status: {}", processed_checkout.status);
            println!("   Transaction ID: {:?}", processed_checkout.transaction_id);
            println!("   Transaction Code: {:?}", processed_checkout.transaction_code);
            
            // Check if 3DS redirect is required
            if let Some(redirect_url) = &processed_checkout.redirect_url {
                println!("\n=== 3DS REDIRECT REQUIRED ===");
                println!("🔐 3D Secure authentication required!");
                println!("Redirect URL: {}", redirect_url);
                println!("\nCustomer should be redirected to this URL to complete 3DS authentication.");
                println!("After authentication, they'll be redirected back to your return_url.");
                
                println!("\n=== 3DS FLOW ===");
                println!("1. Redirect customer to: {}", redirect_url);
                println!("2. Customer authenticates with their bank");
                println!("3. Bank redirects to your return_url with status");
                println!("4. Check checkout status for final result");
            } else {
                println!("\n=== NO 3DS REQUIRED ===");
                println!("Payment processed without 3DS authentication.");
                println!("Final status: {}", processed_checkout.status);
            }
            
            // Show how to check the final status
            println!("\n=== CHECKING FINAL STATUS ===");
            match client.retrieve_checkout(&checkout.id).await {
                Ok(final_checkout) => {
                    println!("Final checkout status: {}", final_checkout.status);
                    if !final_checkout.transactions.is_empty() {
                        for transaction in &final_checkout.transactions {
                            println!("Transaction: {} - {}", transaction.id, transaction.status.as_deref().unwrap_or("Unknown"));
                        }
                    } else {
                        println!("No transactions found");
                    }
                }
                Err(e) => {
                    println!("Error checking final status: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Payment processing failed: {}", e);
            println!("\nThis could be due to:");
            println!("- Invalid test card number");
            println!("- Sandbox environment limitations");
            println!("- Card declined by issuer");
            println!("- Missing required fields");
        }
    }
    
    Ok(())
} 