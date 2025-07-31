use sumup_rs::{SumUpClient, CreateCheckoutRequest, ProcessCheckoutRequest, CardDetails};
use std::io::{self, Write};

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
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("3ds-demo-{}", chrono::Utc::now().timestamp()),
        amount: 5.00, // Small amount for testing
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("3DS Payment Demo".to_string()),
        return_url: Some("https://example.com/payment-return".to_string()),
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
    
    // Process payment with a test card
    let process_request = ProcessCheckoutRequest {
        payment_type: "card".to_string(),
        installments: None,
        card: Some(CardDetails {
            number: "4000000000003220".to_string(), // Test card that may trigger 3DS
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvc: "123".to_string(),
            name: Some("Test Customer".to_string()),
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
                display_3ds_info(redirect_url, &checkout.id);
            } else {
                println!("🎉 Payment completed without 3DS!");
                if let Some(transaction_id) = &processed_checkout.transaction_id {
                    println!("   Transaction ID: {}", transaction_id);
                }
            }
        }
        Ok(sumup_rs::ProcessCheckoutResponse::Accepted(accepted)) => {
            println!("✅ Payment accepted - 3DS required!");
            display_3ds_info(&accepted.next_step.url, &checkout.id);
        }
        Err(e) => {
            println!("❌ Payment processing failed: {}", e);
            println!("\n📋 SANDBOX BEHAVIOR:");
            println!("   - Test cards often fail in sandbox");
            println!("   - This is normal for testing");
            println!("   - Real 3DS testing requires production");
            println!();
            println!("💡 WHAT A REAL 3DS FLOW LOOKS LIKE:");
            println!("   1. Create checkout (✅ done)");
            println!("   2. Process payment with real card");
            println!("   3. Get 3DS redirect URL");
            println!("   4. Customer completes authentication");
            println!("   5. Payment status updated to PAID");
        }
    }
    
    // Wait for user input
    wait_for_user_input();
    
    // Check final status
    check_payment_status(&client, &checkout.id).await;
    
    Ok(())
}

fn display_3ds_info(redirect_url: &str, checkout_id: &str) {
    println!("\n{}", "=".repeat(60));
    println!("🔐 3D SECURE AUTHENTICATION");
    println!("{}", "=".repeat(60));
    println!();
    println!("🌐 3DS URL:");
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