use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");
    
    println!("🚀 MANUAL 3DS TESTING DEMO");
    println!("===========================");
    println!();
    println!("This demo creates a checkout for manual 3DS testing.");
    println!("You can use real cards or test cards in the browser.");
    println!();
    
    // Create a sandbox client
    let client = SumUpClient::new(api_key, true)?;
    
    // Get the merchant profile
    let merchant_profile = client.get_merchant_profile().await?;
    println!("✅ Using merchant code: {}", merchant_profile.merchant_code);
    println!("✅ Currency: {}", merchant_profile.currency);
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("manual-3ds-{}", chrono::Utc::now().timestamp()),
        amount: 50.00,
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("Manual 3DS Testing".to_string()),
        return_url: Some("https://webhook.site/your-unique-url".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    println!("\n🔄 Creating checkout for manual testing...");
    
    // Create the checkout
    let checkout = client.create_checkout(&checkout_request).await?;
    println!("✅ Checkout created!");
    println!("   ID: {}", checkout.id);
    println!("   Amount: {} {}", checkout.amount, checkout.currency);
    println!("   Status: {}", checkout.status);
    
    println!("\n{}", "=".repeat(60));
    println!("🔐 MANUAL 3DS TESTING");
    println!("{}", "=".repeat(60));
    println!();
    println!("🌐 Checkout URL:");
    println!("https://checkout.sumup.com/{}", checkout.id);
    println!();
    println!("📋 Test Cards to Try:");
    println!("   - 4242424242424242 (Visa - Standard)");
    println!("   - 5555555555554444 (Mastercard - Standard)");
    println!("   - 4000000000003220 (Visa - 3DS Required)");
    println!("   - 4000000000009995 (Visa - 3DS Insufficient Funds)");
    println!("   - 4000000000000002 (Visa - Declined)");
    println!();
    println!("📝 Instructions:");
    println!("1. Open the checkout URL in your browser");
    println!("2. Enter any of the test card numbers above");
    println!("3. Use any future expiry date (MM/YY)");
    println!("4. Use any 3-digit CVC");
    println!("5. Complete the payment flow");
    println!("6. If 3DS is required, you'll be redirected");
    println!();
    println!("🔧 Monitoring:");
    println!("   - Check your webhook URL for return calls");
    println!("   - Monitor payment status below");
    println!();
    println!("{}", "=".repeat(60));
    
    // Wait for user input
    println!("\n⏳ Press Enter to monitor payment status...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    
    // Monitor payment status
    monitor_payment_status(&client, &checkout.id).await;
    
    Ok(())
}

async fn monitor_payment_status(client: &SumUpClient, checkout_id: &str) {
    println!("\n📊 PAYMENT STATUS MONITORING");
    println!("{}", "=".repeat(50));
    
    let mut attempts = 0;
    let max_attempts = 20;
    
    while attempts < max_attempts {
        attempts += 1;
        println!("\n🔄 Checking status (attempt {}/{}):", attempts, max_attempts);
        
        match client.retrieve_checkout(checkout_id).await {
            Ok(checkout) => {
                println!("   Status: {}", checkout.status);
                println!("   Amount: {} {}", checkout.amount, checkout.currency);
                
                if !checkout.transactions.is_empty() {
                    println!("   Transactions: {}", checkout.transactions.len());
                    for transaction in &checkout.transactions {
                        println!("     - ID: {}, Status: {}", 
                            transaction.id, 
                            transaction.status.as_deref().unwrap_or("Unknown"));
                    }
                }
                
                match checkout.status.as_str() {
                    "PAID" => {
                        println!("🎉 Payment successful!");
                        break;
                    }
                    "FAILED" => {
                        println!("❌ Payment failed");
                        break;
                    }
                    "CANCELLED" => {
                        println!("🚫 Payment cancelled");
                        break;
                    }
                    "PENDING" => {
                        println!("⏳ Payment still pending...");
                        if attempts < max_attempts {
                            println!("   Waiting 10 seconds before next check...");
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        }
                    }
                    _ => {
                        println!("❓ Unknown status: {}", checkout.status);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("❌ Error checking status: {}", e);
                break;
            }
        }
    }
    
    if attempts >= max_attempts {
        println!("\n⏰ Monitoring timeout reached.");
        println!("   Payment may still be processing.");
        println!("   Check your webhook URL for updates.");
    }
    
    println!("\n{}", "=".repeat(50));
} 