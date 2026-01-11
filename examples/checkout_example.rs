use sumup_rs::{
    Amount, CardDetails, CreateCheckoutRequest, ProcessCheckoutRequest, ProcessCheckoutResponse,
    SumUpClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")?;
    let client = SumUpClient::new(api_key, true)?;

    #[allow(deprecated)]
    let profile = client.get_merchant_profile().await?;
    println!("Merchant: {}", profile.merchant_code);

    // Create checkout (2550 cents = $25.50)
    let request = CreateCheckoutRequest::new(
        format!("order-{}", chrono::Utc::now().timestamp()),
        Amount::from_cents(2550),
        &profile.currency,
        &profile.merchant_code,
    )
    .description("Test payment");

    let checkout = client.create_checkout(&request).await?;
    println!("Created: {} ({})", checkout.id, checkout.status);

    // Process with card
    let process = ProcessCheckoutRequest::card(
        CardDetails::new("4111111111111111", "12", "2025", "123").name("Test User"),
    );

    match client.process_checkout(&checkout.id, &process).await? {
        ProcessCheckoutResponse::Success(c) => println!("Paid: {}", c.status),
        ProcessCheckoutResponse::Accepted(a) => println!("3DS required: {}", a.next_step.url),
    }

    // Cleanup
    client.deactivate_checkout(&checkout.id).await?;
    println!("Deactivated");

    Ok(())
}
