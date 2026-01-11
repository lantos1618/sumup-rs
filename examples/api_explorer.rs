use sumup_rs::{payouts::PayoutListQuery, SumUpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")?;
    let client = SumUpClient::new(api_key, true)?;

    #[allow(deprecated)]
    let profile = client.get_merchant_profile().await?;
    let mc = &profile.merchant_code;
    println!("Merchant: {} ({})", profile.name, mc);

    // Memberships
    let memberships = client.list_memberships().await?;
    println!("Memberships: {}", memberships.len());

    // Checkouts
    let checkouts = client.list_checkouts(None).await?;
    println!("Checkouts: {}", checkouts.len());

    // Readers
    let readers = client.list_merchant_readers(mc).await?;
    println!("Readers: {}", readers.items.len());

    // Payouts
    let payouts = client
        .list_merchant_payouts(mc, &PayoutListQuery::new("2024-01-01", "2024-12-31"))
        .await?;
    println!("Payouts: {}", payouts.items.len());

    Ok(())
}
