use sumup_rs::{SumUpClient, CreateCheckoutRequest, ProcessCheckoutRequest, CardDetails};
use std::io::{self, Write};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");
    
    println!("🚀 COMPREHENSIVE 3DS PAYMENT DEMO");
    println!("=================================");
    println!();
    
    // Get webhook URL from user
    let webhook_url = get_webhook_url();
    
    // Create a sandbox client
    let client = SumUpClient::new(api_key, true)?;
    
    // Get the merchant profile
    let merchant_profile = client.get_merchant_profile().await?;
    println!("✅ Using merchant code: {}", merchant_profile.merchant_code);
    println!("✅ Currency: {}", merchant_profile.currency);
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("3ds-comprehensive-{}", chrono::Utc::now().timestamp()),
        amount: 100.00, // Higher amount for better 3DS triggering
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("Comprehensive 3DS Payment Demo".to_string()),
        return_url: Some(webhook_url.clone()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    println!("\n🔄 Creating payment intent...");
    
    // Create the checkout
    let checkout = client.create_checkout(&checkout_request).await?;
    println!("✅ Payment intent created!");
    println!("   ID: {}", checkout.id);
    println!("   Amount: {} {}", checkout.amount, checkout.currency);
    println!("   Status: {}", checkout.status);
    println!("   Return URL: {}", webhook_url);
    
    // Test 3DS cards systematically
    let test_cards = vec![
        ("4000000000003220", "Visa - 3DS Authentication Required"),
        ("4000000000009995", "Visa - 3DS with Insufficient Funds"),
        ("4000000000000002", "Visa - 3DS Declined"),
        ("4000000000009987", "Visa - 3DS Lost Card"),
        ("4000000000009979", "Visa - 3DS Stolen Card"),
    ];
    
    let mut found_3ds = false;
    
    for (card_number, description) in test_cards {
        println!("\n🔄 Testing card: {} ({})", card_number, description);
        
        let process_request = ProcessCheckoutRequest {
            payment_type: "card".to_string(),
            installments: None,
            card: Some(CardDetails {
                number: card_number.to_string(),
                expiry_month: "12".to_string(),
                expiry_year: "2025".to_string(),
                cvv: "123".to_string(),
                name: Some("3DS Test Customer".to_string()),
            }),
            token: None,
            customer_id: None,
            personal_details: None,
        };
        
        match client.process_checkout(&checkout.id, &process_request).await {
            Ok(sumup_rs::ProcessCheckoutResponse::Success(processed_checkout)) => {
                println!("✅ Payment processed!");
                println!("   Status: {}", processed_checkout.status);
                
                if let Some(redirect_url) = &processed_checkout.redirect_url {
                    display_3ds_info(redirect_url, &checkout.id, card_number, &webhook_url);
                    found_3ds = true;
                    break;
                } else {
                    println!("🎉 Payment completed without 3DS!");
                    if let Some(transaction_id) = &processed_checkout.transaction_id {
                        println!("   Transaction ID: {}", transaction_id);
                    }
                }
            }
            Ok(sumup_rs::ProcessCheckoutResponse::Accepted(accepted)) => {
                println!("✅ Payment accepted - 3DS required!");
                display_3ds_info(&accepted.next_step.url, &checkout.id, card_number, &webhook_url);
                found_3ds = true;
                break;
            }
            Err(e) => {
                println!("❌ Payment processing failed: {}", e);
            }
        }
        
        // Small delay between attempts
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
    
    if !found_3ds {
        println!("\n⚠️  No 3DS authentication triggered with test cards.");
        println!("   This is normal in sandbox environments.");
        println!("   Real 3DS testing requires production environment.");
    }
    
    // Monitor payment status
    monitor_payment_status(&client, &checkout.id).await;
    
    Ok(())
}

fn get_webhook_url() -> String {
    println!("🔧 SETUP REQUIRED:");
    println!("To test 3DS properly, you need a webhook URL to capture return calls.");
    println!();
    println!("1. Go to https://webhook.site");
    println!("2. Copy your unique webhook URL");
    println!("3. Enter it below:");
    println!();
    
    print!("🌐 Webhook URL: ");
    io::stdout().flush().unwrap();
    
    let mut webhook_url = String::new();
    io::stdin().read_line(&mut webhook_url).unwrap();
    
    let webhook_url = webhook_url.trim();
    if webhook_url.is_empty() {
        println!("⚠️  Using default webhook URL (may not work properly)");
        "https://webhook.site/your-unique-url".to_string()
    } else {
        webhook_url.to_string()
    }
}

fn display_3ds_info(redirect_url: &str, checkout_id: &str, card_number: &str, webhook_url: &str) {
    println!("\n{}", "=".repeat(70));
    println!("🔐 3D SECURE AUTHENTICATION REQUIRED");
    println!("{}", "=".repeat(70));
    println!();
    println!("💳 Test Card: {}", card_number);
    println!("🌐 3DS Authentication URL:");
    println!("{}", redirect_url);
    println!();
    println!("📋 Checkout ID: {}", checkout_id);
    println!("🔗 Return URL: {}", webhook_url);
    println!();
    println!("📝 3DS FLOW:");
    println!("1. Customer redirected to 3DS URL");
    println!("2. Customer authenticates with bank");
    println!("3. Bank redirects back to return URL");
    println!("4. Payment status updated");
    println!();
    println!("💡 Alternative checkout URL:");
    println!("https://checkout.sumup.com/{}", checkout_id);
    println!();
    println!("🔧 TESTING INSTRUCTIONS:");
    println!("1. Open the 3DS URL in a browser");
    println!("2. Complete the authentication (may be simulated)");
    println!("3. Check your webhook URL for return calls");
    println!("4. Monitor payment status below");
    println!();
    println!("{}", "=".repeat(70));
}

async fn monitor_payment_status(client: &SumUpClient, checkout_id: &str) {
    println!("\n📊 PAYMENT STATUS MONITORING");
    println!("{}", "=".repeat(50));
    
    let mut attempts = 0;
    let max_attempts = 10;
    
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
                            println!("   Waiting 5 seconds before next check...");
                            tokio::time::sleep(Duration::from_secs(5)).await;
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