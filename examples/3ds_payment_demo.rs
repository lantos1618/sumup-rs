use sumup_rs::{SumUpClient, CreateCheckoutRequest, ProcessCheckoutRequest, CardDetails};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");
    
    println!("🚀 3DS PAYMENT DEMO");
    println!("Testing 3DS payment flow with SumUp sandbox");
    println!();
    
    // Create a sandbox client
    let client = SumUpClient::new(api_key, true)?;
    
    // Get the merchant profile
    let merchant_profile = client.get_merchant_profile().await?;
    println!("✅ Using merchant code: {}", merchant_profile.merchant_code);
    println!("✅ Currency: {}", merchant_profile.currency);
    
    // Create a checkout request with a proper return URL
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("3ds-demo-{}", chrono::Utc::now().timestamp()),
        amount: 100.00, // Higher amount to increase 3DS likelihood
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("3DS Payment Demo - Test Authentication".to_string()),
        return_url: Some("https://webhook.site/your-unique-url".to_string()), // Use a real webhook URL
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
    
    // Try different strategies to trigger 3DS
    println!("\n🔍 TRYING DIFFERENT 3DS TRIGGERING STRATEGIES:");
    println!("1. Using standard test cards");
    println!("2. Using higher amounts");
    println!("3. Using different card types");
    println!();
    
    // Strategy 1: Standard test cards that might work
    let standard_cards = vec![
        ("4242424242424242", "Visa - Standard Success"),
        ("5555555555554444", "Mastercard - Standard Success"),
        ("4000000000000002", "Visa - Declined"),
    ];
    
    // Strategy 2: Cards that might trigger 3DS
    let potential_3ds_cards = vec![
        ("4000000000003220", "Visa - 3DS Authentication Required"),
        ("4000000000009995", "Visa - 3DS with Insufficient Funds"),
        ("4000000000009987", "Visa - 3DS Lost Card"),
        ("4000000000009979", "Visa - 3DS Stolen Card"),
    ];
    
    // Strategy 3: Try with different amounts
    let test_amounts = vec![50.00, 100.00, 200.00];
    
    let mut found_3ds = false;
    
    // Try different amounts first
    for amount in test_amounts {
        if found_3ds { break; }
        
        println!("\n💰 Testing with amount: {} {}", amount, merchant_profile.currency);
        
        // Create a new checkout with this amount
        let amount_checkout_request = CreateCheckoutRequest {
            checkout_reference: format!("3ds-amount-{}-{}", amount, chrono::Utc::now().timestamp()),
            amount,
            currency: merchant_profile.currency.clone(),
            merchant_code: merchant_profile.merchant_code.clone(),
            description: Some(format!("3DS Test - Amount {}", amount)),
            return_url: Some("https://webhook.site/your-unique-url".to_string()),
            customer_id: None,
            purpose: None,
            redirect_url: None,
        };
        
        let amount_checkout = client.create_checkout(&amount_checkout_request).await?;
        println!("   Created checkout: {}", amount_checkout.id);
        
        // Try standard cards first
        for (card_number, description) in &standard_cards {
            if found_3ds { break; }
            
            println!("   Testing: {} ({})", card_number, description);
            
            let process_request = ProcessCheckoutRequest {
                payment_type: "card".to_string(),
                installments: None,
                card: Some(CardDetails {
                    number: card_number.to_string(),
                    expiry_month: "12".to_string(),
                    expiry_year: "2025".to_string(),
                    cvc: "123".to_string(),
                    name: Some("3DS Test Customer".to_string()),
                }),
                token: None,
                customer_id: None,
                personal_details: None,
            };
            
            match client.process_checkout(&amount_checkout.id, &process_request).await {
                Ok(sumup_rs::ProcessCheckoutResponse::Success(processed_checkout)) => {
                    println!("   ✅ Payment processed!");
                    println!("      Status: {}", processed_checkout.status);
                    
                    if let Some(redirect_url) = &processed_checkout.redirect_url {
                        display_3ds_info(redirect_url, &amount_checkout.id, card_number);
                        found_3ds = true;
                        break;
                    } else {
                        println!("   🎉 Payment completed without 3DS!");
                        if let Some(transaction_id) = &processed_checkout.transaction_id {
                            println!("      Transaction ID: {}", transaction_id);
                        }
                    }
                }
                Ok(sumup_rs::ProcessCheckoutResponse::Accepted(accepted)) => {
                    println!("   ✅ Payment accepted - 3DS required!");
                    display_3ds_info(&accepted.next_step.url, &amount_checkout.id, card_number);
                    found_3ds = true;
                    break;
                }
                Err(e) => {
                    println!("   ❌ Payment failed: {}", e);
                }
            }
            
            // Small delay between attempts
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        // If no 3DS with standard cards, try potential 3DS cards
        if !found_3ds {
            for (card_number, description) in &potential_3ds_cards {
                if found_3ds { break; }
                
                println!("   Testing 3DS card: {} ({})", card_number, description);
                
                let process_request = ProcessCheckoutRequest {
                    payment_type: "card".to_string(),
                    installments: None,
                    card: Some(CardDetails {
                        number: card_number.to_string(),
                        expiry_month: "12".to_string(),
                        expiry_year: "2025".to_string(),
                        cvc: "123".to_string(),
                        name: Some("3DS Test Customer".to_string()),
                    }),
                    token: None,
                    customer_id: None,
                    personal_details: None,
                };
                
                match client.process_checkout(&amount_checkout.id, &process_request).await {
                    Ok(sumup_rs::ProcessCheckoutResponse::Success(processed_checkout)) => {
                        println!("   ✅ Payment processed!");
                        println!("      Status: {}", processed_checkout.status);
                        
                        if let Some(redirect_url) = &processed_checkout.redirect_url {
                            display_3ds_info(redirect_url, &amount_checkout.id, card_number);
                            found_3ds = true;
                            break;
                        } else {
                            println!("   🎉 Payment completed without 3DS!");
                            if let Some(transaction_id) = &processed_checkout.transaction_id {
                                println!("      Transaction ID: {}", transaction_id);
                            }
                        }
                    }
                    Ok(sumup_rs::ProcessCheckoutResponse::Accepted(accepted)) => {
                        println!("   ✅ Payment accepted - 3DS required!");
                        display_3ds_info(&accepted.next_step.url, &amount_checkout.id, card_number);
                        found_3ds = true;
                        break;
                    }
                    Err(e) => {
                        println!("   ❌ Payment failed: {}", e);
                    }
                }
                
                // Small delay between attempts
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }
    
    if !found_3ds {
        println!("\n⚠️  SANDBOX 3DS LIMITATIONS:");
        println!("   - Test cards often fail before triggering 3DS");
        println!("   - Sandbox environment may not fully support 3DS");
        println!("   - Real 3DS testing requires production environment");
        println!();
        println!("💡 ALTERNATIVE APPROACHES:");
        println!("   1. Use production environment with real cards");
        println!("   2. Contact SumUp support for 3DS test cards");
        println!("   3. Test 3DS flow manually with real payment");
        println!();
        println!("🔧 DEMO CHECKOUT URL:");
        println!("   https://checkout.sumup.com/{}", checkout.id);
    }
    
    // Wait for user input
    wait_for_user_input();
    
    // Check final status
    check_payment_status(&client, &checkout.id).await;
    
    Ok(())
}

fn display_3ds_info(redirect_url: &str, checkout_id: &str, card_number: &str) {
    println!("\n{}", "=".repeat(60));
    println!("🔐 3D SECURE AUTHENTICATION REQUIRED");
    println!("{}", "=".repeat(60));
    println!();
    println!("💳 Test Card: {}", card_number);
    println!("🌐 3DS Authentication URL:");
    println!("{}", redirect_url);
    println!();
    println!("📋 Checkout ID: {}", checkout_id);
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
    println!("🔧 SANDBOX TESTING:");
    println!("   - Use webhook.site to capture return URL calls");
    println!("   - 3DS authentication may be simulated in sandbox");
    println!("   - Real 3DS testing requires production environment");
    println!();
    println!("{}", "=".repeat(60));
}

fn wait_for_user_input() {
    println!("\n⏳ Press Enter to check payment status...");
    io::stdin().read_line(&mut String::new()).unwrap();
    println!("🔄 Checking payment status...");
}

async fn check_payment_status(client: &SumUpClient, checkout_id: &str) {
    println!("\n{}", "=".repeat(50));
    println!("📊 PAYMENT STATUS");
    println!("{}", "=".repeat(50));
    
    match client.retrieve_checkout(checkout_id).await {
        Ok(checkout) => {
            println!("✅ Status: {}", checkout.status);
            println!("💰 Amount: {} {}", checkout.amount, checkout.currency);
            println!("📅 Created: {}", checkout.date);
            
            if !checkout.transactions.is_empty() {
                println!("\n📋 Transactions:");
                for (i, transaction) in checkout.transactions.iter().enumerate() {
                    println!("   {}. ID: {}", i + 1, transaction.id);
                    println!("      Status: {}", transaction.status.as_deref().unwrap_or("Unknown"));
                    println!("      Amount: {} {}", transaction.amount, transaction.currency);
                    println!();
                }
            }
            
            match checkout.status.as_str() {
                "PAID" => println!("🎉 Payment successful!"),
                "PENDING" => println!("⏳ Payment pending..."),
                "FAILED" => println!("❌ Payment failed"),
                "CANCELLED" => println!("🚫 Payment cancelled"),
                _ => println!("❓ Unknown status"),
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    println!("\n{}", "=".repeat(50));
} 