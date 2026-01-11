use sumup_rs::SumUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")?;

    let client = SumUpClient::new(api_key, true)?;

    #[allow(deprecated)]
    let profile = client.get_merchant_profile().await?;

    println!("API key valid");
    println!("Merchant: {} ({})", profile.name, profile.merchant_code);
    println!("Country: {}", profile.country);
    println!("Currency: {}", profile.currency);

    Ok(())
}
