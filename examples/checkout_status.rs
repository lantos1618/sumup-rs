use sumup_rs::SumUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")?;
    let checkout_id = std::env::args().nth(1).expect("Usage: checkout_status <checkout_id>");

    let client = SumUpClient::new(api_key, true)?;
    let checkout = client.retrieve_checkout(&checkout_id).await?;

    println!("Checkout: {}", checkout.id);
    println!("Status: {}", checkout.status);
    println!("Amount: {} {}", checkout.amount, checkout.currency);

    for txn in &checkout.transactions {
        println!(
            "Transaction: {} ({})",
            txn.id,
            txn.status.as_ref().map(|s| s.to_string()).unwrap_or_default()
        );
    }

    Ok(())
}
