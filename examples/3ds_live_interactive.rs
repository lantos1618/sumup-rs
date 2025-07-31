use sumup_rs::{SumUpClient, CreateCheckoutRequest, ProcessCheckoutRequest, CardDetails};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");
    
    println!("🚀 LIVE 3DS PAYMENT DEMO");
    println!("========================");
    println!();
    println!("⚠️  IMPORTANT: This will process a REAL payment!");
    println!("   Using PRODUCTION environment");
    println!();
    
    // Confirm before proceeding
    print!("Continue with LIVE payment? (yes/no): ");
    io::stdout().flush().unwrap();
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();
    
    if !confirm.trim().eq_ignore_ascii_case("yes") {
        println!("Cancelled.");
        return Ok(());
    }
    
    // Create a PRODUCTION client (false = not sandbox)
    let client = SumUpClient::new(api_key, false)?;
    
    // Get the merchant profile
    let merchant_profile = client.get_merchant_profile().await?;
    println!("\n✅ Using merchant code: {}", merchant_profile.merchant_code);
    println!("✅ Currency: {}", merchant_profile.currency);
    println!("✅ Environment: PRODUCTION");
    
    // Get payment amount
    print!("\n💰 Enter payment amount (e.g., 10.00): ");
    io::stdout().flush().unwrap();
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).unwrap();
    let amount: f64 = amount_str.trim().parse().unwrap_or(10.00);
    
    // Get webhook URL
    println!("\n🔗 Webhook URL Setup:");
    println!("   1. Go to https://webhook.site");
    println!("   2. Copy your unique URL");
    print!("   3. Paste it here: ");
    io::stdout().flush().unwrap();
    let mut webhook_url = String::new();
    io::stdin().read_line(&mut webhook_url).unwrap();
    let webhook_url = webhook_url.trim();
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("live-3ds-{}", chrono::Utc::now().timestamp()),
        amount,
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("Live 3DS Payment Test".to_string()),
        return_url: Some(webhook_url.to_string()),
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
    
    // Get card details
    println!("\n💳 CARD DETAILS:");
    println!("   Enter your REAL card information");
    println!("   This will process an actual payment!");
    println!();
    
    print!("Card number: ");
    io::stdout().flush().unwrap();
    let mut card_number = String::new();
    io::stdin().read_line(&mut card_number).unwrap();
    
    print!("Expiry month (MM): ");
    io::stdout().flush().unwrap();
    let mut expiry_month = String::new();
    io::stdin().read_line(&mut expiry_month).unwrap();
    
    print!("Expiry year (YY): ");
    io::stdout().flush().unwrap();
    let mut expiry_year = String::new();
    io::stdin().read_line(&mut expiry_year).unwrap();
    
    print!("CVV: ");
    io::stdout().flush().unwrap();
    let mut cvv = String::new();
    io::stdin().read_line(&mut cvv).unwrap();
    
    print!("Cardholder name: ");
    io::stdout().flush().unwrap();
    let mut cardholder_name = String::new();
    io::stdin().read_line(&mut cardholder_name).unwrap();
    
    // Process payment
    let process_request = ProcessCheckoutRequest {
        payment_type: "card".to_string(),
        installments: None,
        card: Some(CardDetails {
            number: card_number.trim().replace(" ", ""),
            expiry_month: expiry_month.trim().to_string(),
            expiry_year: expiry_year.trim().to_string(),
            cvc: cvv.trim().to_string(),
            name: Some(cardholder_name.trim().to_string()),
        }),
        token: None,
        customer_id: None,
        personal_details: None,
    };
    
    println!("\n🔄 Processing payment...");
    
    match client.process_checkout(&checkout.id, &process_request).await {
        Ok(sumup_rs::ProcessCheckoutResponse::Success(processed_checkout)) => {
            println!("✅ Payment processed!");
            println!("   Status: {}", processed_checkout.status);
            
            if let Some(redirect_url) = &processed_checkout.redirect_url {
                display_3ds_info(redirect_url, &checkout.id, &webhook_url);
                wait_for_3ds_completion();
            } else {
                println!("🎉 Payment completed without 3DS!");
                if let Some(transaction_id) = &processed_checkout.transaction_id {
                    println!("   Transaction ID: {}", transaction_id);
                }
            }
        }
        Ok(sumup_rs::ProcessCheckoutResponse::Accepted(accepted)) => {
            println!("✅ Payment accepted - 3DS AUTHENTICATION REQUIRED!");
            display_3ds_info(&accepted.next_step.url, &checkout.id, &webhook_url);
            wait_for_3ds_completion();
        }
        Err(e) => {
            println!("❌ Payment processing failed: {}", e);
            println!("   Please check your card details and try again.");
        }
    }
    
    // Monitor payment status
    monitor_payment_status(&client, &checkout.id).await;
    
    Ok(())
}

fn display_3ds_info(redirect_url: &str, checkout_id: &str, webhook_url: &str) {
    println!("\n{}", "=".repeat(70));
    println!("🔐 3D SECURE AUTHENTICATION REQUIRED");
    println!("{}", "=".repeat(70));
    println!();
    println!("🌐 3DS Authentication URL:");
    println!("{}", redirect_url);
    println!();
    println!("📋 Checkout ID: {}", checkout_id);
    println!("🔗 Return URL: {}", webhook_url);
    println!();
    println!("📝 NEXT STEPS:");
    println!("1. Open the 3DS URL above in your browser");
    println!("2. Complete authentication with your bank");
    println!("3. You'll be redirected to your webhook URL");
    println!("4. Check webhook.site for the return data");
    println!();
    println!("💡 Alternative checkout URL:");
    println!("https://checkout.sumup.com/{}", checkout_id);
    println!();
    println!("{}", "=".repeat(70));
}

fn wait_for_3ds_completion() {
    println!("\n⏳ Complete 3DS authentication in your browser...");
    println!("   Press Enter when done to check payment status...");
    io::stdin().read_line(&mut String::new()).unwrap();
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
                    println!("\n   📋 Transactions:");
                    for (i, transaction) in checkout.transactions.iter().enumerate() {
                        println!("   {}. ID: {}", i + 1, transaction.id);
                        println!("      Status: {}", transaction.status.as_deref().unwrap_or("Unknown"));
                        println!("      Amount: {} {}", transaction.amount, transaction.currency);
                        
                        // Show 3DS info if available
                        if let Some(entry_mode) = &transaction.entry_mode {
                            println!("      Entry Mode: {}", entry_mode);
                        }
                        if let Some(auth_code) = &transaction.auth_code {
                            println!("      Auth Code: {}", auth_code);
                        }
                    }
                }
                
                match checkout.status.as_str() {
                    "PAID" => {
                        println!("\n🎉 Payment successful!");
                        println!("   Your payment has been processed successfully.");
                        break;
                    }
                    "FAILED" => {
                        println!("\n❌ Payment failed");
                        break;
                    }
                    "CANCELLED" => {
                        println!("\n🚫 Payment cancelled");
                        break;
                    }
                    "PENDING" => {
                        println!("\n⏳ Payment still pending...");
                        if attempts < max_attempts {
                            println!("   Waiting 5 seconds before next check...");
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                    _ => {
                        println!("\n❓ Unknown status: {}", checkout.status);
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