#![allow(clippy::type_complexity)]
use sumup_rs::{CardDetails, CreateCheckoutRequest, ProcessCheckoutRequest, SumUpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();

    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");

    println!("🔍 SANDBOX 3DS DEBUGGING");
    println!("========================");
    println!();
    println!("Testing with official SumUp test cards in sandbox environment");
    println!();

    // Create a SANDBOX client (sandbox=true)
    let client = SumUpClient::new(api_key, true)?;

    // Get the merchant profile
    let merchant_profile = client.get_merchant_profile().await?;
    println!("✅ Using merchant code: {}", merchant_profile.merchant_code);
    println!("✅ Currency: {}", merchant_profile.currency);
    println!("✅ Environment: SANDBOX");

    // Create a checkout request
    let checkout_request = CreateCheckoutRequest::new(
        format!("sandbox-debug-{}", chrono::Utc::now().timestamp()),
        10.00,
        merchant_profile.currency.clone(),
        merchant_profile.merchant_code.clone(),
    )
    .description("Sandbox 3DS Debug Test")
    .return_url("https://webhook.site/your-unique-url");

    println!("\n🔄 Creating payment intent...");

    // Create the checkout
    let checkout = client.create_checkout(&checkout_request).await?;
    println!("✅ Payment intent created!");
    println!("   ID: {}", checkout.id);
    println!("   Amount: {} {}", checkout.amount, checkout.currency);
    println!("   Status: {}", checkout.status);

    // Test with official SumUp test cards
    let test_cards = vec![
        ("4200000000000042", "Official SumUp Test Card"),
        ("4242424242424242", "Visa - Standard Success"),
        ("5555555555554444", "Mastercard - Standard Success"),
        ("4000000000000002", "Visa - Declined"),
    ];

    for (card_number, description) in test_cards {
        println!("\n🔄 Testing card: {} ({})", card_number, description);

        let process_request = ProcessCheckoutRequest::card(CardDetails {
            number: card_number.to_string(),
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvv: "123".to_string(),
            name: Some("Test Customer".to_string()),
        });

        match client
            .process_checkout(&checkout.id, &process_request)
            .await
        {
            Ok(sumup_rs::ProcessCheckoutResponse::Success(processed_checkout)) => {
                println!("✅ Payment processed!");
                println!("   Status: {}", processed_checkout.status);

                if let Some(redirect_url) = &processed_checkout.redirect_url {
                    println!("   🔐 3DS URL: {}", redirect_url);
                } else {
                    println!("   🎉 No 3DS required");
                }

                if let Some(transaction_id) = &processed_checkout.transaction_id {
                    println!("   Transaction ID: {}", transaction_id);
                }
            }
            Ok(sumup_rs::ProcessCheckoutResponse::Accepted(accepted)) => {
                println!("✅ Payment accepted - 3DS required!");
                println!("   🔐 3DS URL: {}", accepted.next_step.url);
            }
            Err(e) => {
                println!("❌ Payment failed: {}", e);
            }
        }

        // Small delay between attempts
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }

    println!("\n🔍 DEBUGGING COMPLETE");
    println!("Check the request/response logs above to identify any issues.");

    Ok(())
}
