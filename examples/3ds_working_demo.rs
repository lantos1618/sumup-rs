#![allow(clippy::type_complexity)]
use std::io;
use sumup_rs::{CardDetails, CreateCheckoutRequest, ProcessCheckoutRequest, SumUpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();

    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");

    println!("🚀 WORKING 3DS DEMO");
    println!("Based on successful TypeScript implementation");
    println!();

    // Create a PRODUCTION client (not sandbox) - like the TypeScript version
    let client = SumUpClient::new(api_key, false)?;

    // Get the merchant profile
    let merchant_profile = client.get_merchant_profile().await?;
    println!("✅ Using merchant code: {}", merchant_profile.merchant_code);
    println!("✅ Currency: {}", merchant_profile.currency);
    println!("✅ Environment: PRODUCTION");

    // Create a checkout request - matching the TypeScript approach
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: format!("SUMUP-TEST-{}", chrono::Utc::now().timestamp()),
        amount: 2.00,                // Same amount as TypeScript example
        currency: "GBP".to_string(), // Explicitly set to GBP like TypeScript
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("3DS Test Payment".to_string()),
        return_url: Some("https://webhook.site/your-unique-url".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };

    println!("\n🔄 Creating checkout...");

    // Create the checkout
    let checkout = client.create_checkout(&checkout_request).await?;
    println!("✅ Checkout created!");
    println!("   ID: {}", checkout.id);
    println!("   Amount: {} {}", checkout.amount, checkout.currency);
    println!("   Status: {}", checkout.status);

    // Process payment with the EXACT same card details from TypeScript
    let process_request = ProcessCheckoutRequest {
        payment_type: "card".to_string(),
        installments: None,
        card: Some(CardDetails {
            // Test card details for 3DS authentication
            number: "4000000000003220".to_string(), // 3DS test card
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvv: "123".to_string(),
            name: Some("Test Customer".to_string()),
        }),
        token: None,
        customer_id: None,
        personal_details: None,
    };

    println!("\n🔄 Processing payment...");
    println!("   Card: 4000000000003220");
    println!("   Expiry: 12/2025");
    println!("   CVV: 123");
    println!("   Name: Test Customer");

    // Add timeout to prevent hanging
    let process_future = client.process_checkout(&checkout.id, &process_request);
    let timeout_duration = tokio::time::Duration::from_secs(30); // 30 second timeout

    match tokio::time::timeout(timeout_duration, process_future).await {
        Ok(result) => {
            match result {
                Ok(sumup_rs::ProcessCheckoutResponse::Success(processed_checkout)) => {
                    println!("✅ Payment processed!");
                    println!("   Status: {}", processed_checkout.status);

                    if let Some(redirect_url) = &processed_checkout.redirect_url {
                        display_3ds_success(redirect_url, &checkout.id);
                    } else {
                        println!("🎉 Payment completed without 3DS!");
                        if let Some(transaction_id) = &processed_checkout.transaction_id {
                            println!("   Transaction ID: {}", transaction_id);
                        }
                    }
                }
                Ok(sumup_rs::ProcessCheckoutResponse::Accepted(accepted)) => {
                    println!("✅ Payment accepted - 3DS required!");
                    println!("   This matches the TypeScript response!");
                    display_3ds_success(&accepted.next_step.url, &checkout.id);

                    // Log the full response like TypeScript
                    println!("\n📋 FULL RESPONSE (like TypeScript):");
                    println!("{{");
                    println!("  next_step: {{");
                    println!("    url: \"{}\"", accepted.next_step.url);
                    println!("    method: \"{}\"", accepted.next_step.method);
                    if let Some(redirect_url) = &accepted.next_step.redirect_url {
                        println!("    redirect_url: \"{}\"", redirect_url);
                    }
                    if !accepted.next_step.mechanism.is_empty() {
                        println!("    mechanism: {:?}", accepted.next_step.mechanism);
                    }
                    println!("  }}");
                    println!("}}");
                }
                Err(e) => {
                    println!("❌ Payment processing failed: {}", e);
                    println!("\n💡 This might be because:");
                    println!("   - Card details have changed");
                    println!("   - Different merchant account");
                    println!("   - API key differences");
                    println!("   - Network issues");
                    return Ok(());
                }
            }
        }
        Err(_) => {
            println!("⏰ Payment processing timed out after 30 seconds");
            println!("   This could be due to:");
            println!("   - Network connectivity issues");
            println!("   - SumUp API being slow");
            println!("   - Firewall blocking the request");
            println!();
            println!("🔄 Let's try checking the checkout status directly...");

            // Try to get the checkout status to see what happened
            match client.retrieve_checkout(&checkout.id).await {
                Ok(checkout_status) => {
                    println!("📊 Current checkout status: {}", checkout_status.status);
                    if !checkout_status.transactions.is_empty() {
                        println!("📋 Transactions found:");
                        for transaction in &checkout_status.transactions {
                            println!(
                                "   - ID: {}, Status: {}",
                                transaction.id,
                                transaction.status.as_deref().unwrap_or("Unknown")
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Could not retrieve checkout status: {}", e);
                }
            }
            return Ok(());
        }
    }

    // Wait for user input
    wait_for_user_input();

    // Check payment status
    check_payment_status(&client, &checkout.id).await;

    Ok(())
}

fn display_3ds_success(redirect_url: &str, checkout_id: &str) {
    println!("\n{}", "=".repeat(60));
    println!("🎉 3DS SUCCESSFULLY TRIGGERED!");
    println!("{}", "=".repeat(60));
    println!();
    println!("🌐 3DS Authentication URL:");
    println!("{}", redirect_url);
    println!();
    println!("📋 Checkout ID: {}", checkout_id);
    println!();
    println!("📝 NEXT STEPS:");
    println!("1. Open the 3DS URL in your browser");
    println!("2. Complete the authentication");
    println!("3. You'll be redirected back");
    println!("4. The program will monitor payment status automatically");
    println!();
    println!("💡 Alternative checkout URL:");
    println!("https://checkout.sumup.com/{}", checkout_id);
    println!();
    println!("🔧 This matches the TypeScript SDK behavior!");
    println!();
    println!("{}", "=".repeat(60));
}

fn wait_for_user_input() {
    println!("\n⏳ Press Enter when you've opened the 3DS URL...");
    io::stdin().read_line(&mut String::new()).unwrap();
    println!("🔄 Starting payment status monitoring...");
}

async fn check_payment_status(client: &SumUpClient, checkout_id: &str) {
    println!("\n{}", "=".repeat(50));
    println!("📊 PAYMENT STATUS MONITORING");
    println!("{}", "=".repeat(50));

    let mut attempts = 0;
    let max_attempts = 60; // Monitor for up to 10 minutes (60 * 10 seconds)

    while attempts < max_attempts {
        attempts += 1;
        println!(
            "\n🔄 Checking status (attempt {}/{}):",
            attempts, max_attempts
        );

        match client.retrieve_checkout(checkout_id).await {
            Ok(checkout) => {
                println!("   Status: {}", checkout.status);
                println!("   Amount: {} {}", checkout.amount, checkout.currency);

                if !checkout.transactions.is_empty() {
                    for transaction in &checkout.transactions {
                        println!(
                            "   Transaction: {} - {}",
                            transaction.id,
                            transaction.status.as_deref().unwrap_or("Unknown")
                        );
                    }
                }

                match checkout.status.as_str() {
                    "PAID" => {
                        println!("🎉 PAYMENT SUCCESSFUL!");
                        println!("   💰 3DS authentication completed!");
                        println!("   💳 Payment has been processed!");
                        return;
                    }
                    "FAILED" => {
                        println!("❌ Payment failed");
                        println!("   This could be due to:");
                        println!("   - 3DS authentication failed");
                        println!("   - Card declined");
                        println!("   - Insufficient funds");
                        return;
                    }
                    "CANCELLED" => {
                        println!("🚫 Payment cancelled");
                        return;
                    }
                    "PENDING" => {
                        println!("⏳ Payment still pending...");
                        println!("   Waiting for 3DS authentication to complete...");
                        if attempts < max_attempts {
                            println!("   Checking again in 10 seconds...");
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        }
                    }
                    _ => {
                        println!("❓ Unknown status: {}", checkout.status);
                        return;
                    }
                }
            }
            Err(e) => {
                println!("   ⚠️  Status check issue: {}", e);
                println!("   🔄 Retrying in 5 seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }

    if attempts >= max_attempts {
        println!("\n⏰ Monitoring timeout reached (10 minutes).");
        println!("   Payment may still be processing.");
        println!("   Check your bank statement for the charge.");
        println!("   You can manually check the status later.");
    }

    println!("\n{}", "=".repeat(50));
}
