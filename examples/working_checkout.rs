#![allow(clippy::type_complexity)]
use sumup_rs::{
    CardDetails, CheckoutStatus, CreateCheckoutRequest, ProcessCheckoutRequest, ProcessCheckoutResponse,
    SumUpClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();
    let api_key = std::env::var("SUMUP_API_SECRET_KEY").expect("SUMUP_API_SECRET_KEY must be set");

    // Use sandbox=true for testing
    let client = SumUpClient::new(api_key, true)?;
    println!("Client created for SumUp Sandbox API.");

    // 1. Get merchant profile to use the correct merchant_code and currency
    println!("\n1. Fetching merchant profile...");
    let merchant_profile = client.get_merchant_profile().await?;
    println!(
        "   ✅ Found Merchant: {} ({})",
        merchant_profile.name, merchant_profile.merchant_code
    );

    // 2. Create a checkout
    println!("\n2. Creating a new checkout...");
    let checkout_request = CreateCheckoutRequest::new(
        format!("rust-sdk-test-{}", chrono::Utc::now().timestamp()),
        15.75,
        merchant_profile.currency.clone(),
        merchant_profile.merchant_code.clone(),
    )
    .description("Test purchase from Rust SDK")
    .return_url("https://example.com/payment/success");

    let checkout = client.create_checkout(&checkout_request).await?;
    println!("   ✅ Checkout created with ID: {}", checkout.id);

    // 3. Process the checkout with test card details
    println!("\n3. Processing the checkout with a test card...");
    let process_request = ProcessCheckoutRequest::card(
        CardDetails::new("4242424242424242", "12", "2025", "123")
            .name("Test Customer")
    );

    match client
        .process_checkout(&checkout.id, &process_request)
        .await
    {
        Ok(ProcessCheckoutResponse::Success(processed_checkout)) => {
            match processed_checkout.status {
                CheckoutStatus::Paid => {
                    println!("   ✅ Payment successful!");
                    println!(
                        "      Transaction ID: {}",
                        processed_checkout
                            .transaction_id
                            .as_deref()
                            .unwrap_or("N/A")
                    );
                }
                CheckoutStatus::Failed => {
                    println!("   ❌ Payment failed!");
                    println!("      Status: {}", processed_checkout.status);
                    if !processed_checkout.transactions.is_empty() {
                        println!(
                            "      Transaction: {} - {}",
                            processed_checkout.transactions[0].id,
                            processed_checkout.transactions[0]
                                .status
                                .as_ref()
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "Unknown".to_string())
                        );
                    }
                }
                CheckoutStatus::Pending => {
                    println!("   ⏳ Payment pending...");
                    println!("      Status: {}", processed_checkout.status);
                }
                _ => {
                    println!("   ❓ Payment status: {}", processed_checkout.status);
                }
            }

            // Check if there's a redirect URL for 3DS
            if let Some(redirect_url) = &processed_checkout.redirect_url {
                display_3ds_link(redirect_url, &checkout.id);
            }
        }
        Ok(ProcessCheckoutResponse::Accepted(accepted_response)) => {
            println!("   ⚠️ Payment requires 3DS authentication.");
            display_3ds_link(&accepted_response.next_step.url, &checkout.id);
        }
        Err(e) => {
            eprintln!("   ❌ Payment processing failed: {}", e);
        }
    }

    Ok(())
}

fn display_3ds_link(redirect_url: &str, checkout_id: &str) {
    println!("\n{}", "=".repeat(60));
    println!("🎉 3DS AUTHENTICATION REQUIRED!");
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
    println!("4. Check your payment status");
    println!();
    println!("💡 Alternative checkout URL:");
    println!("https://checkout.sumup.com/{}", checkout_id);
    println!();
    println!("{}", "=".repeat(60));
}
