#![allow(clippy::type_complexity)]
use sumup_rs::{
    CardDetails, CreateCheckoutRequest, ProcessCheckoutRequest, ProcessCheckoutResponse,
    SumUpClient,
};

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
    let checkout_request = CreateCheckoutRequest::new(
        format!("order-{}", chrono::Utc::now().timestamp()),
        25.50,
        merchant_profile.currency.clone(),
        merchant_profile.merchant_code.clone(),
    )
    .description("Coffee and pastry")
    .return_url("https://your-app.com/return");

    println!("Creating checkout...");

    // Create the checkout
    match client.create_checkout(&checkout_request).await {
        Ok(checkout) => {
            println!("✅ Checkout created successfully!");
            println!("   ID: {}", checkout.id);
            println!("   Status: {}", checkout.status);
            println!("   Amount: {} {}", checkout.amount, checkout.currency);
            println!("   Reference: {:?}", checkout.checkout_reference);

            // Now let's retrieve the checkout
            println!("\nRetrieving checkout...");
            match client.retrieve_checkout(&checkout.id).await {
                Ok(retrieved_checkout) => {
                    println!("✅ Checkout retrieved successfully!");
                    println!("   ID: {}", retrieved_checkout.id);
                    println!("   Status: {}", retrieved_checkout.status);
                }
                Err(e) => {
                    println!("❌ Failed to retrieve checkout: {}", e);
                }
            }

            // List checkouts with the same reference
            println!(
                "\nListing checkouts with reference '{}'...",
                checkout_request.checkout_reference
            );
            match client
                .list_checkouts(Some(&checkout_request.checkout_reference))
                .await
            {
                Ok(checkouts) => {
                    println!("✅ Found {} checkout(s)", checkouts.len());
                    for (i, checkout) in checkouts.iter().enumerate() {
                        println!(
                            "   {}. ID: {}, Status: {}",
                            i + 1,
                            checkout.id,
                            checkout.status
                        );
                    }
                }
                Err(e) => {
                    println!("❌ Failed to list checkouts: {}", e);
                }
            }

            // Get available payment methods
            println!("\nGetting available payment methods...");
            match client
                .get_available_payment_methods(
                    &merchant_profile.merchant_code,
                    Some(25.50),
                    Some(merchant_profile.currency.as_str()),
                )
                .await
            {
                Ok(methods) => {
                    println!(
                        "✅ Found {} available payment method(s)",
                        methods.available_payment_methods.len()
                    );
                    for method in &methods.available_payment_methods {
                        println!("   - {}", method.id);
                    }
                }
                Err(e) => {
                    println!("❌ Failed to get payment methods: {}", e);
                }
            }

            // Example of processing a checkout
            let process_request = ProcessCheckoutRequest::card(
                CardDetails::new("4111111111111111", "12", "2025", "123")
                    .name("John Doe")
            );

            match client
                .process_checkout(&checkout.id, &process_request)
                .await
            {
                Ok(ProcessCheckoutResponse::Success(processed_checkout)) => {
                    println!("✅ Checkout processed successfully!");
                    println!("   Status: {}", processed_checkout.status);
                }
                Ok(ProcessCheckoutResponse::Accepted(accepted)) => {
                    println!("✅ Checkout accepted - 3DS authentication required!");
                    println!("   Next step URL: {}", accepted.next_step.url);
                }
                Err(e) => {
                    println!("❌ Failed to process checkout: {}", e);
                }
            }

            // Deactivate the checkout
            println!("\nDeactivating checkout...");
            match client.deactivate_checkout(&checkout.id).await {
                Ok(deleted_checkout) => {
                    println!("✅ Checkout deactivated successfully!");
                    println!("   Status: {}", deleted_checkout.status);
                }
                Err(e) => {
                    println!("❌ Failed to deactivate checkout: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create checkout: {}", e);
        }
    }

    Ok(())
}
