use sumup_rs::SumUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("Set SUMUP_API_SECRET_KEY");

    let client = SumUpClient::new(api_key, true)?;

    println!("=== SumUp API Explorer ===\n");

    // Merchant profile
    println!("1. Merchant Profile:");
    let profile = client.get_merchant_profile().await?;
    println!("   Code: {}", profile.merchant_code);
    println!("   Name: {}", profile.name);
    println!("   Country: {}", profile.country);
    println!("   Currency: {}", profile.currency);

    let mc = &profile.merchant_code;

    // Memberships
    println!("\n2. Memberships:");
    match client.list_memberships().await {
        Ok(m) => println!("   Found: {}", m.len()),
        Err(e) => println!("   Error: {}", e),
    }

    // Checkouts
    println!("\n3. Checkouts:");
    match client.list_checkouts(None).await {
        Ok(c) => {
            println!("   Found: {}", c.len());
            for checkout in c.iter().take(3) {
                println!("   - {} {} ({})", checkout.amount, checkout.currency, checkout.status);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    // Readers
    println!("\n4. Readers:");
    match client.list_merchant_readers(mc).await {
        Ok(r) => println!("   Found: {}", r.readers.len()),
        Err(e) => println!("   Error: {}", e),
    }

    // Payouts
    println!("\n5. Payouts:");
    match client.list_merchant_payouts(mc, &sumup_rs::payouts::PayoutListQuery {
        start_date: "2024-01-01".to_string(),
        end_date: "2024-12-31".to_string(),
        limit: Some(5),
        offset: None,
    }).await {
        Ok(p) => println!("   Found: {}", p.payouts.len()),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Done ===");
    Ok(())
}
